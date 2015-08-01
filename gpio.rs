use core::clone::Clone;

use periph::PERIPH_BASE;
use rawstruct::RawStruct;

const APB2PERIPH_BASE: usize = PERIPH_BASE + 0x10000;
pub const GPIOA: usize = APB2PERIPH_BASE + 0x0800;
pub const GPIOB: usize = APB2PERIPH_BASE + 0x0C00;
const GPIOC: usize = APB2PERIPH_BASE + 0x1000;
const GPIOD: usize = APB2PERIPH_BASE + 0x1400;
const GPIOE: usize = APB2PERIPH_BASE + 0x1800;
const GPIOF: usize = APB2PERIPH_BASE + 0x1C00;
const GPIOG: usize = APB2PERIPH_BASE + 0x2000;

const GPIO_MODE_IPD: usize = 0x28;
pub const GPIO_MODE_IPU: usize = 0x48;
const GPIO_MODE_IN_FLOATING: usize = 0x04;
const GPIO_MODE_AIN: usize = 0x0;
const GPIO_MODE_OUT_OD: usize = 0x14;
const GPIO_MODE_OUT_PP: usize = 0x10;
const GPIO_MODE_AF_OD: usize = 0x1C;
const GPIO_MODE_AF_PP: usize = 0x18;

#[derive(Clone)]
struct GPIO {
   CRL: usize,
   CRH: usize,
   IDR: usize,
   ODR: usize,
   BSRR: usize,
   BRR: usize,
   LCKR: usize
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
        let mut tmpreg = 0;
        let mut pos = 0;
        let mut currentpin = 0;
        let mut pinmask = 0;
        let mut currentmode = mode & 0x0F;

        if mode & 0x10 != 0x00 {
            currentmode |= speed;
        }

        self.gpio = Self::from_mem(self.mem_base);

        if pin & 0x00FF != 0x00 {
            tmpreg = self.gpio.CRL;

            for pinpos in 0x00..0x08 {
                pos = 0x01 << pinpos;

                currentpin = pin & pos;
                if currentpin == pos {
                    pos = pinpos << 2;

                    pinmask = 0x0F << pos;
                    tmpreg &= pinmask^(0xFFFFFFFF);
                    tmpreg |= currentmode << pos;

                    match mode {
                        GPIO_MODE_IPD => { self.gpio.BRR = 0x01 << pinpos; },
                        GPIO_MODE_IPU => { self.gpio.BSRR = 0x01 << pinpos; },
                        _ => {}
                    }
                }
            }
            self.gpio.CRL = tmpreg;
        }

        if pin > 0x00FF {
            tmpreg = self.gpio.CRH;
            for pinpos in 0x00..0x08 {
                pos = 0x01 << (pinpos + 0x08);

                currentpin = pin & pos;
                if currentpin == pos {
                    pos = pinpos << 2;

                    pinmask = 0x0F << pos;
                    tmpreg &= pinmask^(0xFFFFFFFF);
                    tmpreg |= currentmode << pos;

                    match mode {
                        GPIO_MODE_IPD => { self.gpio.BRR = 0x01 << (pinpos + 0x08)},
                        GPIO_MODE_IPU => { self.gpio.BSRR = 0x01 << (pinpos + 0x08)},
                        _ => {}
                    }
                }
            }
            self.gpio.CRH = tmpreg;
        }
        self.gpio.BSRR = 0xFFFF;
        self.write(self.mem_base);
    }

    pub fn set(&mut self, pin: usize, val: bool) {
        self.gpio = Self::from_mem(self.mem_base);
        super::util::block(0);
        self.gpio.BSRR = if val {
            pin << 16
        } else {
            pin
        };
        self.write(self.mem_base);
        super::util::block(0);
    }

    pub fn is_set(&mut self, pin: usize) -> bool {
        self.gpio = Self::from_mem(self.mem_base);
        super::util::block(0);
        (self.gpio.IDR & pin) != 0
    }
}
