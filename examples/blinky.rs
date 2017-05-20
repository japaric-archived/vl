//! Blinks an LED

#![feature(const_fn)]
#![feature(used)]
#![no_std]

extern crate cortex_m_rt;
#[macro_use]
extern crate cortex_m_rtfm as rtfm;
extern crate vl;

use rtfm::{Local, P0, P1, T0, T1, TMax};
use vl::stm32f100xx::interrupt::Tim7Irq;
use vl::timer::Timer;
use vl::{led, stm32f100xx};

// CONFIGURATION
pub const FREQUENCY: u32 = 1; // Hz

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
    TIM7: Peripheral {
        register_block: Tim7,
        ceiling: C1,
    },
});

// INITIALIZATION PHASE
fn init(ref prio: P0, thr: &TMax) {
    let gpioc = GPIOC.access(prio, thr);
    let rcc = RCC.access(prio, thr);
    let tim7 = TIM7.access(prio, thr);
    let timer = Timer(&tim7);

    led::init(&gpioc, &rcc);
    timer.init(&rcc, FREQUENCY);
    timer.resume();
}

// IDLE LOOP
fn idle(_prio: P0, _thr: T0) -> ! {
    // Sleep
    loop {
        rtfm::wfi();
    }
}

// TASKS
tasks!(stm32f100xx, {
    periodic: Task {
        interrupt: Tim7Irq,
        priority: P1,
        enabled: true,
    },
});

fn periodic(mut task: Tim7Irq, ref prio: P1, ref thr: T1) {
    static STATE: Local<bool, Tim7Irq> = Local::new(false);

    let tim7 = TIM7.access(prio, thr);
    let timer = Timer(&tim7);

    if timer.clear_update_flag().is_ok() {
        let state = STATE.borrow_mut(&mut task);

        *state = !*state;

        if *state {
            led::BLUE.on();
        } else {
            led::BLUE.off();
        }
    } else {
        // Only reachable through `rtfm::request(periodic)`
        #[cfg(debug_assertions)]
        unreachable!();
    }
}
