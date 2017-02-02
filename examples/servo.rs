//! Puts a servo in position "zero" (1.5 ms)

#![no_std]

extern crate vl;

use vl::servo;

fn main() {
    unsafe {
        servo::initialize(1_500);
        servo::start();
    }
}

#[no_mangle]
pub static _EXCEPTIONS: vl::Exceptions = vl::Exceptions { ..vl::EXCEPTIONS };

#[no_mangle]
pub static _INTERRUPTS: vl::Interrupts = vl::Interrupts { ..vl::INTERRUPTS };
