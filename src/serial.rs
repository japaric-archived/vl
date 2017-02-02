//! PA9 - USART1_TX
//! PA10 - USART1_RX

pub const BAUD_RATE: u32 = 115200;

pub unsafe fn initialize() {
    let afio = ::peripheral::afio_mut();
    let gpioa = ::peripheral::gpioa_mut();
    let nvic = ::cortex_m::peripheral::nvic_mut();
    let rcc = ::peripheral::rcc_mut();
    let usart1 = ::peripheral::usart1_mut();

    // unmask the USART1 interrupt
    nvic.iser[37 / 32].write(1 << (37 % 32));

    // enable GPIOA and USART1
    rcc.apb2enr.modify(|_, w| w.afioen(true).iopaen(true).usart1en(true));

    // wire the PA9 and PA10 pins to USART1
    gpioa.crh.modify(|_, w| w.cnf9(0b10).mode9(0b10).cnf10(0b01));
    afio.mapr.modify(|_, w| w.usart1_remap(false));

    // USART1: 115200 - 8N1
    usart1.cr2.write(|w| w.stop(0b00));

    usart1.brr.write_bits(::APB2_FREQUENCY / BAUD_RATE);

    // disable hardware flow control
    usart1.cr3.write(|w| w.rtse(false).ctse(false));

    usart1.cr1.write(|w| {
        w.ue(true).re(true).te(true).m(false).pce(false).rxneie(true)
    });
}
