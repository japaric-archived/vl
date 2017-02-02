pub fn initialize() {
    let rcc = unsafe { ::peripheral::rcc_mut() };
    let gpioc = unsafe { ::peripheral::gpioc_mut() };

    rcc.apb2enr.modify(|_, w| w.iopcen(true));

    gpioc.crh.modify(|_, w| w.mode8(0b10).cnf8(0b00).mode9(0b10).cnf9(0b00));
}

pub struct Blue;
pub struct Green;

impl Blue {
    pub fn off(&self) {
        let gpioc = unsafe { ::peripheral::gpioc_mut() };

        gpioc.bsrr.write(|w| w.br8(true));
    }

    pub fn on(&self) {
        let gpioc = unsafe { ::peripheral::gpioc_mut() };

        gpioc.bsrr.write(|w| w.bs8(true));
    }
}

impl Green {
    pub fn off(&self) {
        let gpioc = unsafe { ::peripheral::gpioc_mut() };

        gpioc.bsrr.write(|w| w.br9(true));
    }

    pub fn on(&self) {
        let gpioc = unsafe { ::peripheral::gpioc_mut() };

        gpioc.bsrr.write(|w| w.bs9(true));
    }
}
