use core::u16;
use core::any::{Any, TypeId};
use core::ops::Deref;
use stm32f103xx::{RCC, TIM2, TIM3, TIM4, TIM5, tim2};
use cast::{u16, u32};
use hal;
use nb;

const APB1: u32 = 8_000_000;

pub struct Timer<'r, T>(pub &'r T) where T: 'r;

pub unsafe trait TIM_GENERAL: Deref<Target = tim2::RegisterBlock> {}


impl<'a, T> Clone for Timer<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T> Copy for Timer<'a, T> {}

unsafe impl TIM_GENERAL for TIM2 {}
unsafe impl TIM_GENERAL for TIM3 {}
unsafe impl TIM_GENERAL for TIM4 {}
unsafe impl TIM_GENERAL for TIM5 {}

impl<'r, T> Timer<'r, T> where T: TIM_GENERAL + Any {
    pub fn init(&self, rcc: &RCC, frequency: u32) {
        let tim = self.0;
        if tim.get_type_id() == TypeId::of::<TIM2>() {
            rcc.apb1enr.modify(|_,w| w.tim2en().enabled());
        } else if tim.get_type_id() == TypeId::of::<TIM3>() {
            rcc.apb1enr.modify(|_,w| w.tim3en().enabled());
        } else if tim.get_type_id() == TypeId::of::<TIM4>() {
            rcc.apb1enr.modify(|_,w| w.tim2en().enabled());
        } else if tim.get_type_id() == TypeId::of::<TIM5>() {
            rcc.apb1enr.modify(|_,w| w.tim2en().enabled());
        } else {
            unreachable!()
        }
        
        self._set_timeout(frequency);

        self.continuous(true);

		tim.dier.write(|w| w.uie().set());
    }
    
    pub fn continuous(&self, flag: bool) {
        self.0.cr1.write(|w| w.opm().bit(!flag));
    }

    pub fn _set_timeout(&self, frequency: u32) {
        let tim2 = self.0;
        let ratio = APB1 / frequency;
        let psc: u16 = u16(((ratio - 1) / u32(u16::MAX))).expect("overflow on psc");
        tim2.psc.write(|w| w.psc().bits(psc));
        let arr = u16(ratio / (u32(psc) + 1)).expect("overflow on arr");
        tim2.arr.write(|w| w.arr().bits(arr));
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

impl<'a, T> hal::Timer for Timer<'a, T>
    where T: Any + TIM_GENERAL
{
    type Time = u32;

    fn get_timeout(&self) -> u32 {
        unimplemented!();
    }
    
    fn resume(&self) {
        self.0.cr1.modify(|_,w| w.cen().enabled());
    }
    
    fn restart(&self) {
        // Should be reload value.
        self.0.cnt.write(|w| w.cnt().bits(0));
    }

    fn pause(&self) {
        self.0.cr1.modify(|_,w| w.cen().disabled());
    }

    fn set_timeout<TO>(&self, timeout: TO) where TO: Into<u32> {
        self._set_timeout(timeout.into())
    }

    fn wait(&self) -> nb::Result<(), !> {
        if self.0.sr.read().uif().is_clear() {
            Err(nb::Error::WouldBlock)
        } else {
            self.0.sr.modify(|_, w| w.uif().clear());
            Ok(())
        }
    }

}
