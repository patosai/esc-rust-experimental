use stm32f4xx_hal::adc::{
    Adc,
    config::AdcConfig,
    config::SampleTime,
};
use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::stm32::Peripherals;

pub fn enable_adc(p: Peripherals) {
//    TODO
//    let mut adc = Adc::adc1(p.ADC1, true, AdcConfig::default());
//    let gpioc = p.GPIOC.split();
//    gpioc.pc1.into_analog();
//    gpioc.pc2.into_analog();
//    gpioc.pc3.into_analog();
}

fn voltage_millivolts_to_current_amps(millivolts: u16) -> u16 {
    // TODO need to configure DRV8353R first
    return 0;
}

pub fn sample_phase_1_current(p: Peripherals) -> u16 {
    let mut adc = Adc::adc1(p.ADC1, true, AdcConfig::default());
    let gpioc = p.GPIOC.split();
    let pc1 = gpioc.pc1.into_analog();
    let sample = adc.convert(&pc1, SampleTime::Cycles_480);
    let millivolts = adc.sample_to_millivolts(sample);
    return voltage_millivolts_to_current_amps(millivolts);
}
