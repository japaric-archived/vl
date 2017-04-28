//! Serial interface loopback

#![feature(const_fn)]
#![feature(used)]
#![no_std]

extern crate cortex_m_rt;
#[macro_use]
extern crate cortex_m_rtfm as rtfm;
extern crate vl;

use rtfm::{C0, C1, C16, P0, P1};
use vl::serial::Serial;
use vl::stm32f100xx::interrupt::Usart1Irq;
use vl::stm32f100xx;

// CONFIGURATION
pub const BAUD_RATE: u32 = 115_200; // bits per second

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
    RCC: Peripheral {
        register_block: Rcc,
        ceiling: C0,
    },
    USART1: Peripheral {
        register_block: Usart1,
        ceiling: C1,
    },
});

// INITIALIZATION PHASE
fn init(ref prio: P0, ceil: &C16) {
    let afio = AFIO.access(prio, ceil);
    let gpioa = GPIOA.access(prio, ceil);
    let rcc = RCC.access(prio, ceil);
    let usart1 = USART1.access(prio, ceil);

    Serial(&usart1).init(&afio, &gpioa, &rcc, BAUD_RATE);
}

// IDLE LOOP
fn idle(_prio: P0, _ceil: C0) -> ! {
    // Sleep
    loop {
        rtfm::wfi();
    }
}

// TASKS
tasks!(stm32f100xx, {
    loopback: Task {
        interrupt: Usart1Irq,
        priority: P1,
        enabled: true,
    },
});

// Send back the received byte
fn loopback(_task: Usart1Irq, ref prio: P1, ref ceil: C1) {
    let usart1 = USART1.access(prio, ceil);
    let serial = Serial(&usart1);

    if let Ok(byte) = serial.read() {
        if serial.write(byte).is_err() {
            // As we are echoing the bytes as soon as they arrive, it should
            // be impossible to have a TX buffer overrun
            #[cfg(debug_assertions)]
            unreachable!()
        }
    } else {
        // Only reachable through `rtfm::request(loopback)`
        #[cfg(debug_assertions)]
        unreachable!()
    }
}
