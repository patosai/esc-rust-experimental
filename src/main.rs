#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

extern crate stm32f4xx_hal;

//use cortex_m::asm;
use cortex_m_rt::entry;
use stm32f4xx_hal::time::U32Ext;

mod adc;
mod gpio;
mod spi;

#[entry]
fn main() -> ! {
    let p = stm32f4xx_hal::stm32::Peripherals::take().unwrap();
    let clocks = stm32f4xx_hal::prelude::rcc.cfgr.sysclk(48_u32.mhz()).freeze();

    //asm::nop(); // To not have main optimize to abort in release mode, remove when you add code
    gpio::turn_led_1_on(p);

    loop {
        // your code goes here
    }
}
