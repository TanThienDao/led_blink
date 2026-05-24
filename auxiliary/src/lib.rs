//! Initialization code

#![no_std]

pub use panic_itm;
// panic handler

pub use cortex_m::iprintln;
pub use cortex_m::peripheral::ITM;
pub use cortex_m_rt::entry;

pub use stm32f3_discovery::{leds::Leds, stm32f3xx_hal, switch_hal};
pub use switch_hal::{ActiveHigh, OutputSwitch, Switch, ToggleableOutputSwitch};

mod logging;
pub use logging::LogContext;

use stm32f3xx_hal::prelude::*;
pub use stm32f3xx_hal::{
    delay::Delay,
    gpio::{gpioe, Output, PushPull},
    hal::blocking::delay::DelayMs,
    pac,
};

pub type LedArray = [Switch<gpioe::PEx<Output<PushPull>>, ActiveHigh>; 8];

pub fn init() -> (ITM, Delay, LedArray, LogContext) {
    let device_periphs = pac::Peripherals::take().unwrap();
    let mut reset_and_clock_control = device_periphs.RCC.constrain();

    let core_periphs = cortex_m::Peripherals::take().unwrap();
    let mut flash = device_periphs.FLASH.constrain();

    let clocks = reset_and_clock_control.cfgr.freeze(&mut flash.acr);

    //ITM and SYST mustt come from the same one-time core peripheral instance
    let itm = core_periphs.ITM;

    let delay = Delay::new(core_periphs.SYST, clocks);

    // initialize user leds
    let mut gpioe = device_periphs.GPIOE.split(&mut reset_and_clock_control.ahb);
    let leds = Leds::new(
        gpioe.pe8,
        gpioe.pe9,
        gpioe.pe10,
        gpioe.pe11,
        gpioe.pe12,
        gpioe.pe13,
        gpioe.pe14,
        gpioe.pe15,
        &mut gpioe.moder,
        &mut gpioe.otyper,
    );

    let log_ctx = LogContext::new();

    (itm, delay, leds.into_array(), log_ctx)
}
