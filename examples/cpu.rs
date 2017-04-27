//! CPU load monitor

#![feature(const_fn)]
#![feature(used)]
#![no_std]

extern crate byteorder;
extern crate cortex_m_rt;
#[macro_use]
extern crate cortex_m_rtfm as rtfm;
extern crate vl;

use core::cell::Cell;

use byteorder::{ByteOrder, LittleEndian};
use rtfm::{C0, C1, C16, P0, P1, Resource};
use vl::dma::Buffer;
use vl::serial::Serial;
use vl::stm32f100xx::interrupt::{Dma1Channel4Irq, Tim7Irq};
use vl::stm32f100xx;
use vl::timer::Timer;

// CONFIGURATION
pub const BAUD_RATE: u32 = 115_200; // bits per second
const FREQUENCY: u32 = 1;

// RESOURCES
peripherals!(stm32f100xx, {
    AFIO: Peripheral {
        register_block: Afio,
        ceiling: C0,
    },
    DMA1: Peripheral {
        register_block: Dma1,
        ceiling: C1,
    },
    DWT: Peripheral {
        register_block: Dwt,
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
    TIM7: Peripheral {
        register_block: Tim7,
        ceiling: C1,
    },
    USART1: Peripheral {
        register_block: Usart1,
        ceiling: C1,
    },
});

static SLEEP_CYCLES: Resource<Cell<u32>, C1> = Resource::new(Cell::new(0));
static TX_BUFFER: Resource<Buffer<[u8; 4], Dma1Channel4Irq>, C1> =
    Resource::new(Buffer::new([0; 4]));

// INITIALIZATION PHASE
fn init(ref prio: P0, ceil: &C16) {
    let afio = AFIO.access(prio, ceil);
    let dma1 = DMA1.access(prio, ceil);
    let dwt = DWT.access(prio, ceil);
    let gpioa = GPIOA.access(prio, ceil);
    let rcc = RCC.access(prio, ceil);
    let tim7 = TIM7.access(prio, ceil);
    let timer = Timer(&tim7);
    let usart1 = USART1.access(prio, ceil);

    // Enable cycle counter
    unsafe { dwt.ctrl.modify(|r| r | 1) };

    Serial(&usart1).init(&afio, &dma1, &gpioa, &rcc, BAUD_RATE);
    timer.init(&rcc, FREQUENCY);
    timer.resume();
}

// IDLE LOOP
fn idle(ref prio: P0, _ceil: C0) -> ! {
    loop {
        rtfm::atomic(
            |ceil| {
                let dwt = DWT.access(prio, ceil);
                let sleep_cycles = SLEEP_CYCLES.access(prio, ceil);

                let before = dwt.cyccnt.read();
                // Sleep
                rtfm::wfi();
                let after = dwt.cyccnt.read();

                sleep_cycles.set(
                    sleep_cycles.get() +
                    (after.wrapping_sub(before)),
                );
            },
        );

        // Interrupts get serviced here
    }
}

// TASKS
tasks!(stm32f100xx, {
    log: Task {
        interrupt: Tim7Irq,
        priority: P1,
        enabled: true,
    },
    transfer_done: Task {
        interrupt: Dma1Channel4Irq,
        priority: P1,
        enabled: true,
    },
});

fn log(_task: Tim7Irq, ref prio: P1, ref ceil: C1) {
    let dma1 = DMA1.access(prio, ceil);
    let sleep_cycles = SLEEP_CYCLES.access(prio, ceil);
    let tim7 = TIM7.access(prio, ceil);
    let tx_buffer = TX_BUFFER.access(prio, ceil);
    let usart1 = USART1.access(prio, ceil);

    if Timer(&tim7).clear_update_flag().is_ok() {
        LittleEndian::write_u32(
            tx_buffer.borrow().as_mut(),
            sleep_cycles.get(),
        );
        sleep_cycles.set(0);
        Serial(&usart1).write_all(&dma1, tx_buffer);
    } else {
        // This is only reachable through `rtfm::request(log)`
        #[cfg(debug_assertions)]
        unreachable!()
    }
}

fn transfer_done(task: Dma1Channel4Irq, ref prio: P1, ref ceil: C1) {
    let dma1 = DMA1.access(prio, ceil);
    let tx_buffer = TX_BUFFER.access(prio, ceil);

    tx_buffer.release(&task, &dma1);
}
