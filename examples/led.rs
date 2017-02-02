#![no_std]

extern crate vl;

use vl::led;

fn main() {
    led::Blue.on();
    led::Green.on();
}

#[no_mangle]
pub static _EXCEPTIONS: vl::Exceptions = vl::Exceptions { ..vl::EXCEPTIONS };

#[no_mangle]
pub static _INTERRUPTS: vl::Interrupts = vl::Interrupts { ..vl::INTERRUPTS };
