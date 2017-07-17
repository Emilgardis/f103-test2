use stm32f103xx as f103;

pub fn init(gpioc: &f103::GPIOC, rcc: &f103::RCC) {
    rcc.apb2enr.modify(|_,w| w.iopcen().enabled());

    gpioc.crh.modify(|_,w| w
                        .mode13().output50()
                        .cnf13().push()
    );
}

pub struct Led {
    pub i: u8,
}

impl Led {
    pub fn off(&self) {
        unsafe {(*f103::GPIOC.get()).bsrr.write(|w| w.bits(1 << (self.i + 16)))} // Set br field
    }
    
    pub fn on(&self) {
        unsafe {(*f103::GPIOC.get()).bsrr.write(|w| w.bits(1 << (self.i)))} // Set bs field
    }
}

