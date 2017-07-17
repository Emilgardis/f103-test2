use stm32f103xx as f103;
use super::clk;
pub fn init(rcc: &f103::RCC) {
    rcc.cr.reset();
    clk::set_sys_clk_to_hse(rcc);
}
