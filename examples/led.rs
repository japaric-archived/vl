//! Turns all the user LEDs on

#![feature(const_fn)]
#![feature(used)]
#![no_std]

extern crate cortex_m_rt;
#[macro_use]
extern crate cortex_m_rtfm as rtfm;
extern crate vl;

use rtfm::{C0, C16, P0};
use vl::{led, stm32f100xx};

// RESOURCES
peripherals!(stm32f100xx, {
    GPIOC: Peripheral {
        register_block: Gpioc,
        ceiling: C0,
    },
    RCC: Peripheral {
        register_block: Rcc,
        ceiling: C0,
    },
});

// INITIALIZATION PHASE
fn init(ref prio: P0, ceil: &C16) {
    let gpioc = GPIOC.access(prio, ceil);
    let rcc = RCC.access(prio, ceil);

    led::init(&gpioc, &rcc);
}

// IDLE LOOP
fn idle(_prio: P0, _ceil: C0) -> ! {
    led::BLUE.on();
    led::GREEN.on();

    // Sleep
    loop {
        rtfm::wfi();
    }
}

// TASKS
tasks!(stm32f100xx, {});
