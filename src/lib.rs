//! A crate to hack the STM32VLDISCOVERY

#![feature(asm)]
#![feature(compiler_builtins_lib)]
#![feature(const_fn)]
#![feature(core_intrinsics)]
#![feature(lang_items)]
#![feature(macro_reexport)]
#![feature(naked_functions)]
#![no_std]

#[cfg(feature = "semihosting")]
#[macro_reexport(hprint, hprintln)]
#[macro_use]
extern crate cortex_m_semihosting;
extern crate compiler_builtins;
#[macro_reexport(bkpt)]
#[macro_use]
extern crate cortex_m;
extern crate r0;

pub extern crate stm32f100_memory_map as peripheral;

#[macro_use]
mod macros;
mod panicking;
mod rt;

pub mod collections;
pub mod exception;
pub mod led;
pub mod serial;
pub mod servo;

pub const APB1_FREQUENCY: u32 = 8_000_000;
pub const APB2_FREQUENCY: u32 = 8_000_000;

#[repr(C)]
pub struct Exceptions {
    pub nmi: extern "C" fn(),
    pub hard_fault: extern "C" fn(),
    pub mem_manage: extern "C" fn(),
    pub bus_fault: extern "C" fn(),
    pub usage_fault: extern "C" fn(),
    pub _reserved0: [Reserved; 4],
    pub svcall: extern "C" fn(),
    pub debug_monitor: extern "C" fn(),
    pub _reserved1: Reserved,
    pub pendsv: extern "C" fn(),
    pub sys_tick: extern "C" fn(),
}

pub const EXCEPTIONS: Exceptions = Exceptions {
    _reserved0: [Reserved::Vector; 4],
    _reserved1: Reserved::Vector,
    bus_fault: exception::default_handler,
    debug_monitor: exception::default_handler,
    hard_fault: exception::default_handler,
    mem_manage: exception::default_handler,
    nmi: exception::default_handler,
    pendsv: exception::default_handler,
    svcall: exception::default_handler,
    sys_tick: exception::default_handler,
    usage_fault: exception::default_handler,
};

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Reserved {
    Vector = 0,
}

#[repr(C)]
pub struct Interrupts {
    pub wwdg: extern "C" fn(),
    pub pvd: extern "C" fn(),
    pub tamper_stamp: extern "C" fn(),
    pub rtc_wkup: extern "C" fn(),
    pub flash: extern "C" fn(),
    pub rcc: extern "C" fn(),
    pub exti0: extern "C" fn(),
    pub exti1: extern "C" fn(),
    pub exti2: extern "C" fn(),
    pub exti3: extern "C" fn(),
    pub exti4: extern "C" fn(),
    pub dma1_channel1: extern "C" fn(),
    pub dma1_channel2: extern "C" fn(),
    pub dma1_channel3: extern "C" fn(),
    pub dma1_channel4: extern "C" fn(),
    pub dma1_channel5: extern "C" fn(),
    pub dma1_channel6: extern "C" fn(),
    pub dma1_channel7: extern "C" fn(),
    pub adc1: extern "C" fn(),
    pub _reserved0: [Reserved; 4],
    pub exti9_5: extern "C" fn(),
    pub tim1_brk_tim15: extern "C" fn(),
    pub tim1_up_tim16: extern "C" fn(),
    pub tim1_trg_com_tim17: extern "C" fn(),
    pub tim1_cc: extern "C" fn(),
    pub tim2: extern "C" fn(),
    pub tim3: extern "C" fn(),
    pub tim4: extern "C" fn(),
    pub i2c1_ev: extern "C" fn(),
    pub i2c1_er: extern "C" fn(),
    pub i2c2_ev: extern "C" fn(),
    pub i2c2_er: extern "C" fn(),
    pub spi1: extern "C" fn(),
    pub spi2: extern "C" fn(),
    pub usart1: extern "C" fn(),
    pub usart2: extern "C" fn(),
    pub usart3: extern "C" fn(),
    pub exti15_10: extern "C" fn(),
    pub rtc_alarm: extern "C" fn(),
    pub cec: extern "C" fn(),
    pub tim12: extern "C" fn(),
    pub tim13: extern "C" fn(),
    pub tim14: extern "C" fn(),
    pub _reserved1: [Reserved; 2],
    pub fsmc: extern "C" fn(),
    pub _reserved2: Reserved,
    pub tim5: extern "C" fn(),
    pub spi3: extern "C" fn(),
    pub uart4: extern "C" fn(),
    pub uart5: extern "C" fn(),
    pub tim6_dac: extern "C" fn(),
    pub tim7: extern "C" fn(),
    pub dma2_channel1: extern "C" fn(),
    pub dma2_channel2: extern "C" fn(),
    pub dma2_channel3: extern "C" fn(),
    pub dma2_channel4_5: extern "C" fn(),
    pub dma2_channel: extern "C" fn(),
}

pub const INTERRUPTS: Interrupts = Interrupts {
    _reserved0: [Reserved::Vector; 4],
    _reserved1: [Reserved::Vector; 2],
    _reserved2: Reserved::Vector,
    adc1: exception::default_handler,
    cec: exception::default_handler,
    dma1_channel1: exception::default_handler,
    dma1_channel2: exception::default_handler,
    dma1_channel3: exception::default_handler,
    dma1_channel4: exception::default_handler,
    dma1_channel5: exception::default_handler,
    dma1_channel6: exception::default_handler,
    dma1_channel7: exception::default_handler,
    dma2_channel1: exception::default_handler,
    dma2_channel2: exception::default_handler,
    dma2_channel3: exception::default_handler,
    dma2_channel4_5: exception::default_handler,
    dma2_channel: exception::default_handler,
    exti0: exception::default_handler,
    exti15_10: exception::default_handler,
    exti1: exception::default_handler,
    exti2: exception::default_handler,
    exti3: exception::default_handler,
    exti4: exception::default_handler,
    exti9_5: exception::default_handler,
    flash: exception::default_handler,
    fsmc: exception::default_handler,
    i2c1_er: exception::default_handler,
    i2c1_ev: exception::default_handler,
    i2c2_er: exception::default_handler,
    i2c2_ev: exception::default_handler,
    pvd: exception::default_handler,
    rcc: exception::default_handler,
    rtc_alarm: exception::default_handler,
    rtc_wkup: exception::default_handler,
    spi1: exception::default_handler,
    spi2: exception::default_handler,
    spi3: exception::default_handler,
    tamper_stamp: exception::default_handler,
    tim12: exception::default_handler,
    tim13: exception::default_handler,
    tim14: exception::default_handler,
    tim1_brk_tim15: exception::default_handler,
    tim1_cc: exception::default_handler,
    tim1_trg_com_tim17: exception::default_handler,
    tim1_up_tim16: exception::default_handler,
    tim2: exception::default_handler,
    tim3: exception::default_handler,
    tim4: exception::default_handler,
    tim5: exception::default_handler,
    tim6_dac: exception::default_handler,
    tim7: exception::default_handler,
    uart4: exception::default_handler,
    uart5: exception::default_handler,
    usart1: exception::default_handler,
    usart2: exception::default_handler,
    usart3: exception::default_handler,
    wwdg: exception::default_handler,
};
