//! Board Support Crate for the STM32VLDISCOVERY
//!
//! # Features
//!
//! - Periodic timer
//! - Serial interface
//! - User LEDs
//!
//! # Usage
//!
//! Follow `cortex-m-quickstart` [instructions][i], and add this crate as a
//! dependency and remove the `memory.x` linker script and the `build.rs` build
//! script file as part of the configuration of the quickstart crate.
//!
//! [i]: https://docs.rs/cortex-m-quickstart/0.1.8/cortex_m_quickstart/

#![feature(conservative_impl_trait)]
// #![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

pub extern crate stm32f100xx;

extern crate cast;
extern crate futures;

// From tokio-core
macro_rules! try_nb {
    ($e:expr) => {
        match $e {
            Ok(t) => t,
            Err(Error::WouldBlock) => return Ok(Async::NotReady),
            Err(e) => return Err(e.into()),
        }
    }
}

pub mod led;
pub mod serial;
pub mod timer;

mod frequency;
