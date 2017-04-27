//! Blink an LED

#![feature(const_fn)]
#![feature(used)]
#![no_std]

extern crate cortex_m_rt;
#[macro_use]
extern crate cortex_m_rtfm as rtfm;
extern crate vl;

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

static TX_BUFFER: Resource<Buffer<[u8; 15], Dma1Channel4Irq>, C1> =
    Resource::new(Buffer::new([0; 15]));

// INITIALIZATION PHASE
fn init(ref prio: P0, ceil: &C16) {
    let afio = AFIO.access(prio, ceil);
    let dma1 = DMA1.access(prio, ceil);
    let gpioa = GPIOA.access(prio, ceil);
    let rcc = RCC.access(prio, ceil);
    let tim7 = TIM7.access(prio, ceil);
    let timer = Timer(&tim7);
    let tx_buffer = TX_BUFFER.access(prio, ceil);
    let usart1 = USART1.access(prio, ceil);

    tx_buffer.borrow().copy_from_slice(b"Hello, world!\n\r");
    Serial(&usart1).init(&afio, &dma1, &gpioa, &rcc, BAUD_RATE);
    timer.init(&rcc, FREQUENCY);
    timer.resume();
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
    periodic: Task {
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

// Send "Hello, world" through the serial interface every 1 second
fn periodic(_task: Tim7Irq, ref prio: P1, ref ceil: C1) {
    let dma1 = DMA1.access(prio, ceil);
    let tim7 = TIM7.access(prio, ceil);
    let tx_buffer = TX_BUFFER.access(prio, ceil);
    let usart1 = USART1.access(prio, ceil);

    if Timer(&tim7).clear_update_flag().is_ok() {
        // Queues a DMA transfer from TX_BUFFER into USART1_TX
        // this op would PANIC if `tx_buffer` is being used in a DMA transfer
        // this op would PANIC if the DMA channel is already in use (e.g. by
        // some other peripheral)
        // TODO add a `range` parameter to slice the buffer before the transfer
        // NOTE this won't compile if you pass a buffer than has been created
        // in this task stack
        Serial(&usart1).write_all(&dma1, tx_buffer);

        // any further access to `tx_buffer` would result in a PANIC
        // FIXME I now realize this is too restrictive; it's safe to read
        // TX_BUFFER while the transfer is ongoing
    }

}

fn transfer_done(task: Dma1Channel4Irq, ref prio: P1, ref ceil: C1) {
    let dma1 = DMA1.access(prio, ceil);
    let tx_buffer = TX_BUFFER.access(prio, ceil);

    // NOTE the type of `task` must match the signature of `TX_BUFFER`
    // this "releases" the buffer so one can access it again
    // this clears all the interrupt flags related to this channel
    // this op would PANIC if the DMA transfer is not complete
    tx_buffer.release(&task, &dma1);
}
