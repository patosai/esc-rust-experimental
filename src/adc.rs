use stm32f4xx_hal::adc::{
    Adc,
    config::AdcConfig,
    config::SampleTime,
};
use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::stm32::Peripherals;


pub fn enable_adc(p: Peripherals) {
    let mut adc = Adc::adc1(p.ADC1, true, AdcConfig::default());
    let gpioc = p.GPIOC.split();
    let pc1 = gpioc.pc1.into_analog();
    let sample = adc.convert(&pc1, SampleTime::Cycles_480);
    let millivolts = adc.sample_to_millivolts(sample);
}
