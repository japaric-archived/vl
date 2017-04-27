//! Serial interface
//!
//! - TX - PA9
//! - RX - PA10

use core::ptr;

use cast::{u16, u8};
use stm32f100xx::{Afio, Gpioa, Rcc, Usart1};

use frequency;

/// Specialized `Result` type
pub type Result<T> = ::core::result::Result<T, Error>;

/// An error
pub struct Error {
    _0: (),
}

/// Serial interface
///
/// # Interrupts
///
/// - `Usart1Irq` - RXNE (RX buffer not empty)
// - Dma1Channel4 (USART1_TX) - TC (transfer complete)
#[derive(Clone, Copy)]
pub struct Serial<'a>(pub &'a Usart1);

impl<'a> Serial<'a> {
    /// Initializes the serial interface with a baud rate of `baud_rate` bits
    /// per second
    ///
    /// The serial interface will be configured to use 8 bits of data, 1 stop
    /// bit, no hardware control and to omit parity checking
    pub fn init(
        &self,
        afio: &Afio,
        // dma1: &Dma1,
        gpioa: &Gpioa,
        rcc: &Rcc,
        baud_rate: u32,
    ) {
        let usart1 = self.0;

        // Power up peripherals
        // rcc.ahbenr.modify(|_, w| unsafe { w.dma1en().bits(1) });
        rcc.apb2enr
            .modify(
                |_, w| unsafe {
                    w.afioen()
                        .bits(1)
                        .usart1en()
                        .bits(1)
                        .iopaen()
                        .bits(1)
                },
            );

        // Use PA9 and PA10 as TX and RX
        afio.mapr
            .modify(|_, w| unsafe { w.usart1_remap().bits(0) });

        // Configure PA9 as alternate function, push pull output
        // Configure PA10 as input
        gpioa
            .crh
            .modify(
                |_, w| unsafe {
                    w.cnf9()
                        .bits(0b10)
                        .mode9()
                        .bits(0b10)
                        .cnf10()
                        .bits(0b01)
                },
            );


        // USART1_TX
        // Channel4: Memory++ (8-bit) -> Peripheral (8-bit)
        // dma1.ccr4
        //     .modify(
        //         |_, w| unsafe {
        //             w.mem2mem()
        //                 .bits(0)
        //                 .msize()
        //                 .bits(0b00)
        //                 .psize()
        //                 .bits(0b00)
        //                 .minc()
        //                 .bits(1)
        //                 .pinc()
        //                 .bits(0)
        //                 .dir()
        //                 .bits(1)
        //                 .en()
        //                 .bits(0)
        //                 .tcie()
        //                 .bits(1)
        //         },
        //     );

        // 8N1
        usart1.cr2.write(|w| unsafe { w.stop().bits(0) });

        // No CTS / RTS
        usart1
            .cr3
            .write(|w| unsafe {
                w.rtse()
                    .bits(0)
                    .ctse()
                    .bits(0)
                    .dmat()
                    .bits(1)
            });

        // Set baud rate
        let brr = u16(frequency::APB2 / baud_rate).unwrap();
        let mantissa = brr >> 4;
        let fraction = u8(brr & 0b1111).unwrap();
        usart1
            .brr
            .write(
                |w| unsafe {
                    w.div_mantissa()
                        .bits(mantissa)
                        .div_fraction()
                        .bits(fraction)
                },
            );

        // Enable
        usart1
            .cr1
            .write(
                |w| unsafe {
                    w.ue()
                        .bits(1)
                        .re()
                        .bits(1)
                        .te()
                        .bits(1)
                        .m()
                        .bits(0)
                        .pce()
                        .bits(0)
                        .rxneie()
                        .bits(1)
                },
            );
    }

    /// Reads a byte
    ///
    /// This method returns `Err` if there's no byte to read
    pub fn read(&self) -> Result<u8> {
        let usart1 = self.0;

        if usart1.sr.read().rxne().bits() == 1 {
            // NOTE(read_volatile) The DR register is 9 bits long but we are
            // only going to use 8 bits
            Ok(
                unsafe {
                    ptr::read_volatile(&usart1.dr as *const _ as *const u8)
                },
            )
        } else {
            Err(Error { _0: () })
        }
    }

    /// Writes a byte
    ///
    /// This method returns `Err` if the write operation would result in a
    /// buffer overrun
    pub fn write(&self, byte: u8) -> Result<()> {
        let usart1 = self.0;

        if usart1.sr.read().txe().bits() == 1 {
            // NOTE(write_volatile) See NOTE in the `read` method
            unsafe {
                ptr::write_volatile(&usart1.dr as *const _ as *mut u8, byte)
            }
            Ok(())
        } else {
            Err(Error { _0: () })
        }
    }
}
