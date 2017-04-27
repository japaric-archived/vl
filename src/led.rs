//! User LEDs

use stm32f100xx::{GPIOC, Gpioc, Rcc};

/// Blue LED (PC8)
pub static BLUE: Led = Led { i: 8 };

/// Green LED (PC9)
pub static GREEN: Led = Led { i: 9 };

/// An LED
pub struct Led {
    i: u8,
}

impl Led {
    /// Turns off the LED
    pub fn off(&self) {
        // NOTE(safe) atomic write
        unsafe { (*GPIOC.get()).bsrr.write(|w| w.bits(1 << (self.i + 16))) }
    }

    /// Turns on the LED
    pub fn on(&self) {
        // NOTE(safe) atomic write
        unsafe { (*GPIOC.get()).bsrr.write(|w| w.bits(1 << self.i)) }
    }
}

/// Initialize the LED driving pins
pub fn init(gpioc: &Gpioc, rcc: &Rcc) {
    // Power up peripherals
    rcc.apb2enr.modify(|_, w| unsafe { w.iopcen().bits(1) });

    // Configure PC8 and PC9 as general purpose, push pull outputs
    gpioc
        .crh
        .modify(
            |_, w| unsafe {
                w.mode8()
                    .bits(0b01)
                    .cnf8()
                    .bits(0b00)
                    .mode9()
                    .bits(0b01)
                    .cnf9()
                    .bits(0b00)
            },
        )
}
