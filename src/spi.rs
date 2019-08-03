use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::rcc::Clocks;
use stm32f4xx_hal::stm32::Peripherals;
use stm32f4xx_hal::spi;

pub fn enable_spi(p: Peripherals, clocks: Clocks) {
    let gpiob = p.GPIOB.split();
    let tx = gpiob.pb15.into_alternate_af5();
    let rx = gpiob.pb14.into_alternate_af5();

    let spi = spi::Spi(
        p.SPI2,
        (tx, rx),
        (spi::Polarity::IdleHigh, spi::Phase::CaptureOnFirstTransition),
        HERTZ,
        clocks,
    ).unwrap();
}
