#![feature(used, const_fn, never_type, get_type_id)]
#![no_std]

#[macro_use]
extern crate cortex_m;
extern crate cortex_m_rt;
#[macro_use]
extern crate cortex_m_rtfm as rtfm;

extern crate stm32f103xx;

extern crate embedded_hal as hal;
extern crate nb;

extern crate cast;

mod system;
mod led;
mod button;


use rtfm::{Local, P0, P1, P2, T0, T1, T2, TMax};
use hal::Timer;

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
    TIM3: Peripheral {
        register_block: TIM3,
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
    ITM: Peripheral {
        register_block: ITM,
        ceiling: C0,
    },
});


use stm32f103xx::interrupt::TIM2;

fn init(prio: P0, thre: &TMax) {
    let gpiob = GPIOB.access(&prio, thre);
    let gpioc = GPIOC.access(&prio, thre);
    let rcc = RCC.access(&prio, thre);
    let tim2 = TIM2.access(&prio, thre);
    let tim3 = TIM3.access(&prio, thre);
    let exti = EXTI.access(&prio, thre);
    let afio = AFIO.access(&prio, thre);
    system::init::init(&rcc);
    led::init(&gpioc, &rcc);
    let timer = system::timer::Timer(&*tim2);
    timer.init(&rcc, 100);
	timer.resume();
    let button_timer = system::timer::Timer(&*tim3);
    button_timer.init(&rcc, (1.0/0.05) as u32); // 50 ms debounce.
    button_timer.continuous(false);
    button::init(&gpiob, &rcc, &exti, &afio);
}

fn idle(ref prio: P0, ref thres: T0) -> ! {
    let gpiob = stm32f103xx::GPIOB.get();
    let itm = ITM.access(prio, thres);
    loop {
        rtfm::wfi();
    }
}

tasks!(stm32f103xx, {
    periodic: Task {
        interrupt: TIM2,
        priority: P1,
        enabled: true,
    },
    exti9_5: Task {
        interrupt: EXTI9_5,
        priority: P1,
        enabled: true,
    },
});


fn periodic(mut task: TIM2, ref prio: P1, ref thres: T1) {
    static STATE: Local<bool, TIM2> = Local::new(false);
    // Should we check for clear update?
    let tim2 = TIM2.access(prio, thres);
    let timer = system::timer::Timer(&*tim2);
    timer.clear_update_flag().unwrap();
    let state = STATE.borrow_mut(&mut task);
    *state = !*state;
    if *state {
       led::Led{i:13}.on();
    } else {
        led::Led{i:13}.off();
    }
}

fn exti9_5(mut task: stm32f103xx::interrupt::EXTI9_5, ref prio: P1, ref thres: T1) {
    static HZ: Local<u32, stm32f103xx::interrupt::EXTI9_5> = Local::new(100);
    let exti = EXTI.access(prio, thres);
    let tim2 = TIM2.access(prio, thres);
    let timer = system::timer::Timer(&*tim2);
    let tim3 = TIM3.access(prio, thres);
    let button_timer = system::timer::Timer(&*tim3);
    exti.pr.write(|w| w.pr7().set());
    if button_timer.0.cnt.read().cnt().bits() != 0 {
        return;       
    }
    button_timer.resume();
    let hz = HZ.borrow_mut(&mut task);
    if *hz == 1 {
        *hz = 100;
    } else if *hz <= 10 {
        *hz -= 1;
    } else {
        *hz -= 5;
    }
    timer.set_timeout(*hz);
    hprintln!("{}", hz);

    let gpiob = GPIOB.access(prio, thres);
    gpiob.bsrr.write(|w| w.br7().reset());
}
