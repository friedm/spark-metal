use core::clone::Clone;

use periph::PERIPH_BASE;
use rawstruct::RawStruct;

const APB2PERIPH_BASE: usize = PERIPH_BASE + 0x10000;
pub const GPIOA: usize = APB2PERIPH_BASE + 0x0800;
pub const GPIOB: usize = APB2PERIPH_BASE + 0x0C00;

const GPIO_MODE_IPD: usize = 0x28;
pub const GPIO_MODE_IPU: usize = 0x48;

#[derive(Clone)]
struct GPIO {
   crl: usize,
   crh: usize,
   idr: usize,
   odr: usize,
   bsrr: usize,
   brr: usize,
   lckr: usize
}

#[derive(Clone)]
pub struct Gpio {
    mem_base: usize,
    gpio: GPIO
}

impl RawStruct<GPIO> for Gpio {
    fn to_struct(&self) -> GPIO {
        (*self).gpio.clone()
    }
}

impl Gpio {
    pub fn from(mem_base: usize) -> Gpio {
        Gpio {
            mem_base: mem_base,
            gpio: Self::from_mem(mem_base)
        }
    }

    pub fn init_pin(&mut self, pin: usize, mode: usize, speed: usize) {
        let mut currentmode = mode & 0x0F;

        if mode & 0x10 != 0x00 {
            currentmode |= speed;
        }

        self.gpio = Self::from_mem(self.mem_base);

        if pin & 0x00FF != 0x00 {
            let mut tmpreg = self.gpio.crl;

            for pinpos in 0x00..0x08 {
                let mut pos = 0x01 << pinpos;

                let currentpin = pin & pos;
                if currentpin == pos {
                    pos = pinpos << 2;

                    let pinmask = 0x0F << pos;
                    tmpreg &= pinmask^(0xFFFFFFFF);
                    tmpreg |= currentmode << pos;

                    match mode {
                        GPIO_MODE_IPD => { self.gpio.brr = 0x01 << pinpos; },
                        GPIO_MODE_IPU => { self.gpio.bsrr = 0x01 << pinpos; },
                        _ => {}
                    }
                }
            }
            self.gpio.crl = tmpreg;
        }

        if pin > 0x00FF {
            let mut tmpreg = self.gpio.crh;
            for pinpos in 0x00..0x08 {
                let mut pos = 0x01 << (pinpos + 0x08);

                let currentpin = pin & pos;
                if currentpin == pos {
                    pos = pinpos << 2;

                    let pinmask = 0x0F << pos;
                    tmpreg &= pinmask^(0xFFFFFFFF);
                    tmpreg |= currentmode << pos;

                    match mode {
                        GPIO_MODE_IPD => { self.gpio.brr = 0x01 << (pinpos + 0x08)},
                        GPIO_MODE_IPU => { self.gpio.bsrr = 0x01 << (pinpos + 0x08)},
                        _ => {}
                    }
                }
            }
            self.gpio.crh = tmpreg;
        }
        self.gpio.bsrr = 0xFFFF;
        self.write(self.mem_base);
    }

    pub fn set(&mut self, pin: usize, val: bool) {
        self.gpio = Self::from_mem(self.mem_base);
        self.gpio.bsrr = if val {
            pin << 16
        } else {
            pin
        };
        self.write(self.mem_base);
    }

    pub fn is_set(&mut self, pin: usize) -> bool {
        self.gpio = Self::from_mem(self.mem_base);
        (self.gpio.idr & pin) != 0
    }
}
