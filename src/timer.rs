//! Periodic timer

use core::u16;

use cast::{u16, u32};
use stm32f100xx::{Rcc, Tim7};
use futures::future;
use futures::{Async, Future};

use frequency;

/// Specialized `Result` type
pub type Result<T> = ::core::result::Result<T, Error>;

/// An error
pub enum Error {
    WouldBlock,
    #[doc(hidden)]
    _Extensible,
}

/// Periodic timer
///
/// # Interrupts
///
/// - `Tim7Irq` - update event
#[derive(Clone, Copy)]
pub struct Timer<'a>(pub &'a Tim7);

impl<'a> Timer<'a> {
    /// Initializes the timer with a periodic timeout of `frequency` Hz
    ///
    /// NOTE The timer starts in a paused state
    pub fn init(&self, rcc: &Rcc, frequency: u32) {
        let tim7 = self.0;

        rcc.apb1enr.modify(|_, w| unsafe { w.tim7en().bits(1) });

        let ratio = frequency::APB1 / frequency;
        let psc = u16(((ratio - 1) / u32(u16::MAX))).unwrap();
        tim7.psc.write(|w| unsafe { w.psc().bits(psc) });
        let arr = u16(ratio / (u32(psc) + 1)).unwrap();
        tim7.arr.write(|w| unsafe { w.arr().bits(arr) });

        tim7.dier.write(|w| unsafe { w.uie().bits(1) });
    }

    /// Waits for a timeout
    ///
    /// Returns an `Err` if no update event has occurred
    pub fn wait(&self) -> Result<()> {
        let tim7 = self.0;

        if tim7.sr.read().uif().bits() == 0 {
            Err(Error::WouldBlock)
        } else {
            tim7.sr.write(|w| w);
            Ok(())
        }
    }

    /// Pauses the timer
    pub fn pause(&self) {
        let tim7 = self.0;

        tim7.cr1.write(|w| unsafe { w.cen().bits(0) });
    }

    /// Resumes the timer
    pub fn resume(&self) {
        let tim7 = self.0;

        tim7.cr1.write(|w| unsafe { w.cen().bits(1) });
    }
}

/// Waits for timeout (future style)
pub fn wait<'a>(timer: Timer<'a>) -> impl Future<Item = (), Error = Error> + 'a {
    future::poll_fn(move || Ok(Async::Ready(try_nb!(timer.wait()))))
}
