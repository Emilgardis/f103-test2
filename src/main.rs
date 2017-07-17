#![feature(used, const_fn)]
#![no_std]

#[macro_use]
extern crate cortex_m;
extern crate cortex_m_rt;
#[macro_use]
extern crate cortex_m_rtfm as rtfm;

extern crate stm32f103xx;

extern crate cast;

mod system;
mod led;
mod button;
use rtfm::{Local, P0, P1, P2, T0, T1, T2, TMax};

peripherals!(stm32f103xx, {
    GPIOB: Peripheral {
        register_block: GPIOB,
        ceiling: C1,
    },
    GPIOC: Peripheral {
        register_block: GPIOC,
        ceiling: C0,
    },
    RCC: Peripheral {
        register_block: RCC,
        ceiling: C0,
    },
    TIM2: Peripheral {
        register_block: TIM2,
        ceiling: C1,
    },
    EXTI: Peripheral {
        register_block: EXTI,
        ceiling: C1,
    },
    AFIO: Peripheral {
        register_block: AFIO,
        ceiling: C1,
    },
});


use stm32f103xx::interrupt::TIM2;

fn init(prio: P0, thre: &TMax) {
    let gpiob = GPIOB.access(&prio, thre);
    let gpioc = GPIOC.access(&prio, thre);
    let rcc = RCC.access(&prio, thre);
    let tim2 = TIM2.access(&prio, thre);
    let exti = EXTI.access(&prio, thre);
    let afio = AFIO.access(&prio, thre);
    system::init::init(&rcc);
    led::init(&gpioc, &rcc);
    let timer = system::timer::Timer(&tim2);
    timer.init(&rcc, 100);
    button::init(&gpiob, &rcc, &exti, &afio);
}

fn idle(ref prio: P0, ref thres: T0) -> ! {
    let gpiob = stm32f103xx::GPIOB.get();
    let mut hz = 100;
    loop {
        unsafe {
            if (*gpiob).idr.read().idr7().is_set() {
                thres.raise(&TIM2, |threshold: &T1| {
                    let tim2 = TIM2.access(prio, threshold);
                    let timer = system::timer::Timer(&tim2);
                    if hz == 1 {
                        hz = 100;
                    } else if hz <= 10 {
                        hz -= 1;
                    } else {
                        hz -= 5;
                    }
                    timer.set_hz(hz);
                });

            } else {
                rtfm::wfi();
            }
        }
    }
}

tasks!(stm32f103xx, {
    periodic: Task {
        interrupt: TIM2,
        priority: P1,
        enabled: true,
    },
    exti0: Task {
        interrupt: EXTI0,
        priority: P1,
        enabled: false,
    },
});


fn periodic(mut task: TIM2, ref prio: P1, ref thres: T1) {
    static STATE: Local<bool, TIM2> = Local::new(false);
    // Should we check for clear update?
    let tim2 = TIM2.access(prio, thres);
    let timer = system::timer::Timer(&tim2);
    timer.clear_update_flag().unwrap();
    let state = STATE.borrow_mut(&mut task);
    *state = !*state;
    if *state {
       led::Led{i:13}.on();
    } else {
        led::Led{i:13}.off();
    }
}

fn exti0(mut task: stm32f103xx::interrupt::EXTI0, ref prio: P1, ref thres: T1) {
    //hprintln!("Button pressed");
    let exti = EXTI.access(prio, thres);
    let gpiob = GPIOB.access(prio, thres);

    exti.pr.write(|w| w.pr7().clear());
    gpiob.bsrr.write(|w| w.br7().reset());
}
