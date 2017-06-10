//! Two tasks running cooperatively

#![feature(const_fn)]
#![feature(used)]
#![no_std]

extern crate cortex_m_rt;
#[macro_use]
extern crate cortex_m_rtfm as rtfm;
extern crate futures;
extern crate vl;

use futures::Future;
use futures::future::{self, Loop};
use rtfm::{P0, T0, TMax};
use vl::led;
use vl::serial::{self, Serial};
use vl::stm32f100xx;
use vl::timer::{self, Timer};

// CONFIGURATION
const BAUD_RATE: u32 = 115_200; // bits per second
const FREQUENCY: u32 = 1; // Hz

// RESOURCES
peripherals!(stm32f100xx, {
    AFIO: Peripheral {
        register_block: Afio,
        ceiling: C0,
    },
    GPIOA: Peripheral {
        register_block: Gpioa,
        ceiling: C0,
    },
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
        ceiling: C0,
    },
    USART1: Peripheral {
        register_block: Usart1,
        ceiling: C0,
    },
});

// INITIALIZATION PHASE
fn init(ref prio: P0, thr: &TMax) {
    let afio = AFIO.access(prio, thr);
    let gpioa = GPIOA.access(prio, thr);
    let gpioc = GPIOC.access(prio, thr);
    let rcc = RCC.access(prio, thr);
    let tim7 = TIM7.access(prio, thr);
    let usart1 = USART1.access(prio, thr);

    let serial = Serial(&usart1);
    let timer = Timer(&tim7);

    led::init(&gpioc, &rcc);
    serial.init(&afio, &gpioa, &rcc, BAUD_RATE);
    timer.init(&rcc, FREQUENCY);
    timer.resume();
}

// IDLE LOOP
fn idle(ref prio: P0, ref thr: T0) -> ! {
    let tim7 = TIM7.access(prio, thr);
    let usart1 = USART1.access(prio, thr);

    let serial = Serial(&usart1);
    let timer = Timer(&tim7);

    // Blink an LED
    let mut led = future::loop_fn::<_, (), _, _>(
        false, move |mut state| {
            timer::wait(timer).map(
                move |_| {
                    state = !state;

                    if state {
                        led::BLUE.on();
                    } else {
                        led::BLUE.off();
                    }

                    Loop::Continue(state)
                }
            )
        }
    );

    // Send back the received byte
    let mut loopback = future::loop_fn::<_, (), _, _>(
        (), move |_| {
            serial::read(serial).and_then(
                move |byte| {
                    serial::write(serial, byte).map(Loop::Continue)
                }
            )
        }
    );

    // Run the tasks cooperatively
    loop {
        led.poll().ok();
        loopback.poll().ok();
    }
}

// TASKS
tasks!(stm32f100xx, {});
