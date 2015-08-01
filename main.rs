#![feature(no_std)]
#![feature(core)]
#![feature(lang_items)]
#![no_std]

#![crate_type="staticlib"]

extern crate core;

pub mod runtime;
pub mod rawstruct;
pub mod periph;

extern {
    fn c_main() -> ();
}

#[no_mangle]
pub fn main() {
    periph::enable();
    unsafe {
        c_main();
    }
}
