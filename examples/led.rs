//! Turns all the user LEDs on

#![feature(const_fn)]
#![feature(used)]
#![no_std]

extern crate cortex_m_rt;
#[macro_use]
extern crate cortex_m_rtfm as rtfm;
extern crate vl;

use rtfm::{P0, T0, TMax};
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
fn init(ref prio: P0, thr: &TMax) {
    let gpioc = GPIOC.access(prio, thr);
    let rcc = RCC.access(prio, thr);

    led::init(&gpioc, &rcc);
}

// IDLE LOOP
fn idle(_prio: P0, _thr: T0) -> ! {
    led::BLUE.on();
    led::GREEN.on();

    // Sleep
    loop {
        rtfm::wfi();
    }
}

// TASKS
tasks!(stm32f100xx, {});
