use stm32f4xx_hal::stm32::Peripherals;
use stm32f4xx_hal::prelude::*;

pub fn turn_led_1_on(p: Peripherals) {
    let gpioc = p.GPIOC.split();
    gpioc.pc8.into_push_pull_output()
        .set_high();
}
