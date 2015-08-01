#![feature(no_std)]
#![feature(core)]
#![feature(lang_items)]
#![no_std]

#![crate_type="staticlib"]

extern crate core;

pub mod runtime;

mod rawstruct;
mod periph;
mod gpio;
mod led;
mod button;

use gpio::*;
use led::*;
use button::*;

extern {
    fn c_main() -> ();
}

#[no_mangle]
pub fn main() {
    periph::enable();

    let mut gpio_a = Gpio::from(GPIOA);
    gpio_a.init_pin(LED_RED|LED_BLUE|LED_GREEN, 0x10, 3);

    let mut gpio_b = Gpio::from(GPIOB);
    gpio_b.init_pin(BUTTON_PIN, GPIO_MODE_IPU, 2);

    unsafe {
        c_main();
    }
}
