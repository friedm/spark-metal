#![feature(no_std)]
#![feature(core)]
#![feature(lang_items)]
#![no_std]

#![crate_type="staticlib"]

pub mod runtime;

mod rawstruct;
mod periph;
mod gpio;
mod led;
mod button;

mod util;

use gpio::*;
use led::*;
use button::*;

#[no_mangle]
pub fn main() {
    periph::enable();

    let mut led = Led::new(&mut Gpio::from(GPIOA));

    let mut gpio_b = Gpio::from(GPIOB);
    gpio_b.init_pin(BUTTON_PIN, GPIO_MODE_IPU, 2);

    led.set(LED_WHITE);

    let mut rand = util::Rand::from(11110);

    loop {
        let n = (&mut rand).next() as usize;
        if !gpio_b.is_set(BUTTON_PIN) {
            match n%6 {
                0 => led.set(LED_RED),
                1 => led.set(LED_GREEN),
                2 => led.set(LED_BLUE),
                3 => led.set(LED_CYAN),
                4 => led.set(LED_YELLOW),
                5 => led.set(LED_VIOLET),
                _ => {}
            }
            while !gpio_b.is_set(BUTTON_PIN) {}
        }

        util::delay(5000);
    }
}
