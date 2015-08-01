#![feature(no_std)]
#![feature(core)]
#![feature(lang_items)]
#![no_std]

#![crate_type="staticlib"]

extern crate core;

pub mod runtime;

#[no_mangle]
pub fn main() {
    loop { }
}
