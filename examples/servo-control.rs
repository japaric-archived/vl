//! Control a servo using serial port
//!
//! Frame format: A stringified number followed by Carriage Return
//!
//! The number will be parsed as an `u16` value.
//!
//! This number will be the duration, in microseconds, of ON part of the PWM
//! signal.
//!
//! Depending on the length of the pulse, you get a different angle:
//!
//! pulse | angle
//! (us)  | (degrees)
//! ------+----------
//!  800  | -90
//! 1600  |   0
//! 2400  |  90

#![feature(const_fn)]
#![no_std]

extern crate ascii;
extern crate vl;

use ascii::AsAsciiStr;
use vl::{peripheral, serial, servo};
use vl::collections::Vec;

fn main() {
    unsafe {
        servo::initialize(0);
        serial::initialize();
        servo::start();
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
                if let Ok(n) = s.parse::<u16>() {
                    servo::on(n);
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
