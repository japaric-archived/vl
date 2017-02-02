//! Echo back bytes

#![feature(asm)]
#![no_std]

extern crate cortex_m;
extern crate vl;

use vl::{peripheral, serial};

fn main() {
    unsafe {
        serial::initialize();
    }
}

#[no_mangle]
pub static _EXCEPTIONS: vl::Exceptions = vl::Exceptions { ..vl::EXCEPTIONS };

#[no_mangle]
pub static _INTERRUPTS: vl::Interrupts =
    vl::Interrupts { usart1: usart1, ..vl::INTERRUPTS };

extern "C" fn usart1() {
    unsafe {
        let usart1 = peripheral::usart1_mut();

        let dr = usart1.dr.read_bits();
        usart1.dr.write_bits(dr);

        usart1.sr.modify(|_, w| w.rxne(false));
    }
}
