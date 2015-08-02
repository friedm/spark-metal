use core::clone::Clone;

use gpio::Gpio;

pub const LED_BLUE: usize = 0x0100;
pub const LED_GREEN: usize = 0x0400;
pub const LED_RED: usize = 0x0200;
pub const LED_VIOLET: usize = LED_BLUE|LED_RED;
pub const LED_CYAN: usize = LED_BLUE|LED_GREEN;
pub const LED_YELLOW: usize = LED_RED|LED_GREEN;
pub const LED_WHITE: usize = LED_BLUE|LED_RED|LED_GREEN;

const LED_ALL: usize = LED_WHITE;

pub struct Led {
    gpio: Gpio
}

impl Led {
    pub fn new(gpio: &mut Gpio) -> Led {
        gpio.init_pin(LED_RED|LED_BLUE|LED_GREEN, 0x10, 3);
        Led {
            gpio: (*gpio).clone()
        }
    }

    pub fn set(&mut self, color: usize) {
        self.off();
        self.gpio.set(color, true);
    }

    fn off(&mut self) {
        self.gpio.set(LED_ALL, false);
    }
}
