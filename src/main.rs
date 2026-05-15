#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, DelayMs, LedArray, OutputSwitch};

const BLINK_PERIOD_MS: u16 = 200;   // all-LEDs blink delay
const ROULETTE_PERIOD_MS: u16 = 50; // roulette step delay
const BLINK_CYCLES: u8 = 5;         // full on/off blinks before roulette

#[entry]
fn main() -> ! {
    let (mut delay, mut leds) = aux5::init();

    loop {
        // Phase 1: Blink all 8 LEDs on/off together
        for _ in 0..BLINK_CYCLES {
            all_on(&mut leds);
            delay.delay_ms(BLINK_PERIOD_MS);
            all_off(&mut leds);
            delay.delay_ms(BLINK_PERIOD_MS);
        }

        // Phase 2: Roulette — chase one LED around the ring
        for curr in 0..leds.len() {
            let next = (curr + 1) % leds.len();
            leds[next].on().ok();
            delay.delay_ms(ROULETTE_PERIOD_MS);
            leds[curr].off().ok();
        }
    }
}

/// Turn all 8 LEDs on
fn all_on(leds: &mut LedArray) {
    for led in leds.iter_mut() {
        led.on().ok();
    }
}

/// Turn all 8 LEDs off
fn all_off(leds: &mut LedArray) {
    for led in leds.iter_mut() {
        led.off().ok();
    }
}
