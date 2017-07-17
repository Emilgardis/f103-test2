use stm32f103xx as f103;

pub fn set_sys_clk_to_hse(rcc: &f103::RCC) {
    rcc.cr.modify(|_,w| w.hseon().enabled());
    let mut counter = 0;
    while !rcc.cr.read().hserdy().is_ready() {
        counter += 1;
        if counter == 0x0500 {
            break;
        }
    }
    if rcc.cr.read().hserdy().is_ready() {
        //hprintln!("Counter: {}", counter);
        // Enable prefetch buffer flash.acr->prftbe, then wait
        rcc.cfgr.modify(|_,w| w
                        .hpre().div1()
                        .ppre2().div1()
                        .ppre1().div1()
                        .sw().hse()
        );
        counter = 0;
        while !rcc.cr.read().hserdy().is_ready() {
            counter += 1;
            if counter == 0x0500 {
                break;
            }
        }
    } else {
        panic!("Sys Clock couldn't be set to hse!")
    }
}
