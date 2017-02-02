#![no_std]

extern crate vl;

fn main() {}

#[no_mangle]
pub static _EXCEPTIONS: vl::Exceptions = vl::Exceptions { ..vl::EXCEPTIONS };

#[no_mangle]
pub static _INTERRUPTS: vl::Interrupts = vl::Interrupts { ..vl::INTERRUPTS };
