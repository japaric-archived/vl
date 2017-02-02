#![feature(asm)]
#![feature(const_fn)]
#![no_std]

extern crate ascii;
#[macro_use]
extern crate vl;

use ascii::AsAsciiStr;
use vl::collections::Vec;
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
    // Carriage Return
    const CR: u8 = 13;
    static mut BUFFER: Vec<u8, [u8; 16]> = Vec::new([0u8; 16]);

    unsafe {
        let usart1 = peripheral::usart1_mut();
        usart1.sr.modify(|_, w| w.rxne(false));

        let byte = usart1.dr.read_bits() as u8;

        if byte == CR {
            if let Ok(s) = BUFFER.as_ascii_str().map(|s| s.as_str()) {
                if let Ok(_n) = s.parse::<u8>() {
                    bkpt!();
                }
            }

            BUFFER.clear();
        } else {
            if BUFFER.push(byte).is_err() {
                BUFFER.clear()
            }
        }
    }
}
