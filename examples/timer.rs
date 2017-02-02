#![no_std]

extern crate cortex_m;
extern crate vl;

use vl::{led, peripheral};

const TIMER_FREQUENCY: u32 = 1_000;

fn main() {
    let nvic = unsafe { cortex_m::peripheral::nvic_mut() };
    let rcc = unsafe { peripheral::rcc_mut() };
    let tim6 = unsafe { peripheral::tim6_mut() };

    // unmask the tim6 interrupt
    nvic.iser[54 / 32].write(1 << (54 % 32));

    // enable TIM6
    rcc.apb1enr.modify(|_, w| w.tim6en(true));

    // configure for 1 Hz interrupt
    tim6.psc
        .write(|w| w.psc(((vl::APB1_FREQUENCY / TIMER_FREQUENCY) - 1) as u16));
    tim6.arr.write(|w| w.arr(TIMER_FREQUENCY as u16));

    // clear the update event flag
    tim6.sr.read();
    tim6.sr.write(|w| w);

    // enable update interrupts
    tim6.dier.write(|w| w.uie(true));

    // configure as periodic and start
    tim6.cr1.write(|w| w.cen(true).opm(false));
}

#[no_mangle]
pub static _EXCEPTIONS: vl::Exceptions = vl::Exceptions { ..vl::EXCEPTIONS };

#[no_mangle]
pub static _INTERRUPTS: vl::Interrupts =
    vl::Interrupts { tim6_dac: tim6, ..vl::INTERRUPTS };

extern "C" fn tim6() {
    static mut ON: bool = false;

    unsafe {
        // clear the update event flag
        peripheral::tim6_mut().sr.write(|w| w);

        ON = !ON;

        if ON {
            led::Blue.on();
        } else {
            led::Blue.off();
        }
    }
}
