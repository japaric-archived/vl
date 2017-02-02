//! PA0 - TIM2_CH1 (PWM)

pub const FREQUENCY: u32 = 300;

pub unsafe fn initialize(us: u16) {
    let afio = ::peripheral::afio_mut();
    let gpioa = ::peripheral::gpioa_mut();
    let rcc = ::peripheral::rcc_mut();
    let tim2 = ::peripheral::tim2_mut();

    // enable AFIO, TIM2 and GPIOA
    rcc.apb1enr.modify(|_, w| w.tim2en(true));
    rcc.apb2enr.modify(|_, w| w.afioen(true).iopaen(true));

    // PA0 - alternate push pull
    gpioa.crl.modify(|_, w| w.cnf0(0b10).mode0(0b10));

    // wire TIM2_CH1 to PA0
    afio.mapr.modify(|_, w| w.tim2_remap(0b00));

    // PWM mode 1
    tim2.ccmr1_output.modify(|_, w| w.oc1pe(true).oc1m(0b110));
    tim2.ccer.modify(|_, w| w.cc1p(false).cc1e(true));

    tim2.psc.write(|w| w.psc(0));
    tim2.arr.write(|w| w.arr((::APB1_FREQUENCY / FREQUENCY) as u16));
    tim2.ccr1.write(|w| w.ccr1(us * (::APB1_FREQUENCY / 1_000_000) as u16));

    tim2.cr1.write(|w| w.cms(0b00).dir(false).opm(false).cen(false));
}

pub fn on(us: u16) {
    unsafe {
        ::peripheral::tim2_mut()
            .ccr1
            .write(|w| w.ccr1(us * (::APB1_FREQUENCY / 1_000_000) as u16));
    }
}

pub fn start() {
    unsafe {
        ::peripheral::tim2_mut().cr1.modify(|_, w| w.cen(true));
    }
}

pub fn stop() {
    unsafe {
        ::peripheral::tim2_mut().cr1.modify(|_, w| w.cen(false));
    }
}
