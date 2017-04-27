
//! DMA

use core::cell::{Cell, UnsafeCell};
use core::marker::PhantomData;
use core::ops;

use stm32f100xx::dma1;

/// Identifies an interrupt handler associated to a DMA channel
///
/// DO NOT IMPLEMENT THIS TRAIT YOURSELF
pub unsafe trait Channel {
    /// DMA associated to this channel
    type Dma: ops::Deref<Target = dma1::RegisterBlock>;

    /// Mark the transfer as done
    unsafe fn done(&self, dma: &Self::Dma);
}

unsafe impl Channel for ::stm32f100xx::interrupt::Dma1Channel4Irq {
    type Dma = ::stm32f100xx::Dma1;

    unsafe fn done(&self, dma: &::stm32f100xx::Dma1) {
        assert!(dma.isr.read().tcif4().bits() == 1, "transfer not complete");
        dma.ifcr.write(|w| w.cgif4().bits(1));
    }
}

/// Buffer associated to DMA channel `C` and backed by the buffer `B`
pub struct Buffer<B, C> {
    _channel: PhantomData<C>,
    borrowed: Cell<bool>,
    buffer: UnsafeCell<B>,
}

/// Wrapper for a mutably borrowed value from a `Buffer`
pub struct RefMut<'b, B>
where
    B: 'b,
{
    borrowed: &'b Cell<bool>,
    buffer: &'b mut B,
}

impl<B, C> Buffer<B, C> {
    /// Creates a new `Buffer`
    pub const fn new(buffer: B) -> Buffer<B, C>
    where
        B: AsRef<[u8]> + AsMut<[u8]>,
        C: Channel,
    {
        Buffer {
            _channel: PhantomData,
            borrowed: Cell::new(false),
            buffer: UnsafeCell::new(buffer),
        }
    }

    /// Mutably borrows the buffer
    pub fn borrow(&self) -> RefMut<B> {
        assert!(!self.borrowed.get(), "Buffer is already in use");
        self.borrowed.set(true);
        RefMut {
            borrowed: &self.borrowed,
            buffer: unsafe { &mut *self.buffer.get() },
        }
    }

    /// Releases the buffer
    pub fn release(&self, task: &C, dma: &C::Dma)
    where
        C: Channel,
    {
        assert!(self.borrowed.get(), "Buffer has already been released");
        unsafe { task.done(dma) }
        self.borrowed.set(false);
    }
}

impl<'b, B> Drop for RefMut<'b, B> {
    fn drop(&mut self) {
        self.borrowed.set(false);
    }
}

impl<'b, B> ops::Deref for RefMut<'b, B> {
    type Target = B;

    fn deref(&self) -> &B {
        self.buffer
    }
}

impl<'b, B> ops::DerefMut for RefMut<'b, B> {
    fn deref_mut(&mut self) -> &mut B {
        self.buffer
    }
}
