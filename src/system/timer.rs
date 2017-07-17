use core::u16;
use stm32f103xx::{RCC,TIM2};
use cast::{u16, u32};


pub struct Timer<'r>(pub &'r TIM2);

const APB1: u32 = 8_000_000;

impl<'r> Timer<'r> {
    pub fn init(&self, rcc: &RCC, frequency: u32) {
        let tim2 = self.0;

        rcc.apb1enr.modify(|_,w| w.tim2en().enabled());

        let ratio = APB1 / frequency;
        let psc = u16(((ratio - 1) / u32(u16::MAX))).unwrap();
        tim2.psc.write(|w| w.psc().bits(psc));
        let arr = u16(ratio / (u32(psc) + 1)).unwrap();
        tim2.arr.write(|w| w.arr().bits(arr));
		tim2.dier.write(|w| w.uie().set());
        tim2.cr1.write(|w| w.opm().continuous());
        self.resume();
        //hprintln!("set tim2 to ratio: {}, psc: {}, arr: {}", ratio, psc, arr,);
    }

    pub fn set_hz(&self, frequency: u32) {
        let tim2 = self.0;
        self.pause();
        tim2.dier.write(|w| w.uie().clear());
        let ratio = APB1 / frequency;
        let psc: u16 = u16(((ratio - 1) / u32(u16::MAX))).expect("Overflow on psc");
        tim2.psc.write(|w| w.psc().bits(psc));
        //::cortex_m::asm::bkpt();
        let arr = u16(ratio / (u32(psc) + 1)).expect("overflow on arr");
        tim2.arr.write(|w| w.arr().bits(arr));
        tim2.dier.write(|w| w.uie().set());
        self.resume();
    }

    pub fn resume(&self) {
        let tim2 = self.0;

        tim2.cr1.write(|w| w.cen().enabled());
    }
    
    pub fn pause(&self) {
        let tim2 = self.0;

        tim2.cr1.write(|w| w.cen().disabled());
    }

    pub fn clear_update_flag(&self) -> ::core::result::Result<(),()> {
        let tim2 = self.0;

        if !tim2.sr.read().uif().bit() {
            //hprintln!("Ehhh");
            Err(())
        } else {
            tim2.sr.reset();
            Ok(())
        }
    }
}
