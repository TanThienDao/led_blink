# ITM Logging Guide (`aux5` + `led-blink`)

This document explains how to add runtime ITM log points to this project, why `ITM` must be initialized inside the same
peripheral init flow, and what better logging options you can adopt later.

---

## Checklist

- [x] Explain current setup in `auxiliary/src/lib.rs` and `src/main.rs`
- [x] Show implementation code (before/after snippets)
- [x] Explain why a separate log init function is problematic
- [x] List future logging alternatives and migration direction
- [x] Keep `README.md` / `openocd.gdb` updates deferred for later

---

## Current State (What you have now)

In `auxiliary/src/lib.rs`:

- `init()` takes device peripherals (`pac::Peripherals::take()`)
- `init()` takes core peripherals (`cortex_m::Peripherals::take()`)
- `SYST` is consumed to build `Delay`
- LEDs are configured and returned
- Return type is currently `(Delay, LedArray)`

In `src/main.rs`:

- `let (mut delay, mut leds) = aux5::init();`
- no runtime ITM log writes yet

---

## Implementation (Code)

### 1) Update `auxiliary/src/lib.rs`

### Before

```rust
pub fn init() -> (Delay, LedArray) {
    let device_periphs = pac::Peripherals::take().unwrap();
    let mut reset_and_clock_control = device_periphs.RCC.constrain();

    let core_periphs = cortex_m::Peripherals::take().unwrap();
    let mut flash = device_periphs.FLASH.constrain();
    let clocks = reset_and_clock_control.cfgr.freeze(&mut flash.acr);
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

    (delay, leds.into_array())
}
```

### After

```rust
pub use cortex_m::peripheral::ITM;

pub fn init() -> (ITM, Delay, LedArray) {
    let device_periphs = pac::Peripherals::take().unwrap();
    let mut reset_and_clock_control = device_periphs.RCC.constrain();

    let core_periphs = cortex_m::Peripherals::take().unwrap();
    let mut flash = device_periphs.FLASH.constrain();
    let clocks = reset_and_clock_control.cfgr.freeze(&mut flash.acr);

    // ITM and SYST must come from the same one-time core peripheral instance.
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

    (itm, delay, leds.into_array())
}
```

### 2) Update `src/main.rs`

If you want to use `cortex_m::iprintln!` directly in the app crate, add `cortex-m` to the top-level `Cargo.toml` first:

```toml
[dependencies]
aux5 = { path = "auxiliary" }
volatile = "0.4.3"
cortex-m = "0.7.2"
```

Then update `src/main.rs`:

```rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, DelayMs, LedArray, OutputSwitch};
use cortex_m::iprintln;

const BLINK_PERIOD_MS: u16 = 200;
const ROULETTE_PERIOD_MS: u16 = 50;
const BLINK_CYCLES: u8 = 5;

#[entry]
fn main() -> ! {
    let (mut itm, mut delay, mut leds) = aux5::init();

    iprintln!(&mut itm.stim[0], "boot: main entered");

    loop {
        iprintln!(&mut itm.stim[0], "phase: blink");
        for _ in 0..BLINK_CYCLES {
            all_on(&mut leds);
            delay.delay_ms(BLINK_PERIOD_MS);
            all_off(&mut leds);
            delay.delay_ms(BLINK_PERIOD_MS);
        }

        iprintln!(&mut itm.stim[0], "phase: roulette");
        for curr in 0..leds.len() {
            let next = (curr + 1) % leds.len();
            leds[next].on().ok();
            delay.delay_ms(ROULETTE_PERIOD_MS);
            leds[curr].off().ok();
        }
    }
}

fn all_on(leds: &mut LedArray) {
    for led in leds.iter_mut() {
        led.on().ok();
    }
}

fn all_off(leds: &mut LedArray) {
    for led in leds.iter_mut() {
        led.off().ok();
    }
}
```

---

## Why you should not create a separate `log_init()` that calls `Peripherals::take()` again

Short answer: **`cortex_m::Peripherals::take()` is a singleton**.

- First successful call returns `Some(Peripherals)`
- Later calls return `None`
- If `init()` already took core peripherals, a later `log_init()` cannot take them again
- Using `.unwrap()` in that second function can panic immediately

So this is fragile:

```rust
pub fn log_init() -> ITM {
    let p = cortex_m::Peripherals::take().unwrap(); // may panic if already taken
    p.ITM
}
```

### The ownership reason

`SYST` and `ITM` are owned fields of the same `Peripherals` struct.
You should split and hand them out once in a single initialization boundary (`init()`), then pass references where
needed.

---

## Can we still use separate functions for logging?

Yes, but not for **taking peripherals**.

You can separate **log writing** logic by passing references to already-owned ITM resources.

Example pattern:

```rust
use cortex_m::iprintln;
use cortex_m::peripheral::ITM;

pub fn log_boot(itm: &mut ITM) {
    iprintln!(&mut itm.stim[0], "boot");
}
```

This is good modularization because it does not violate singleton peripheral ownership.

---

## Better logging alternatives for future

## 1) `defmt` + RTT (recommended next step)

Why this is often better than raw ITM:

- compact, structured logs (very small firmware overhead)
- better host tooling (`probe-rs`, `defmt-print`)
- easier filtering and formatting
- widely used in modern Rust embedded projects

When to choose:

- you want scalable logs as project complexity grows
- you want better developer experience than raw SWO setup

## 2) ITM (current path)

Pros:

- very low latency channel on Cortex-M
- no extra UART wiring needed on many boards

Cons:

- setup can be sensitive (clock/SWO/OpenOCD/GDB pipeline)
- less ergonomic than `defmt` for larger apps

## 3) UART logging

Pros:

- simple and robust in production diagnostics
- easy capture with serial tools

Cons:

- consumes a UART peripheral and pins
- more runtime overhead than RTT/ITM in many cases

## 4) Semihosting

Pros:

- easy for quick bring-up under debugger

Cons:

- very slow, debugger-dependent
- usually not suitable for normal runtime behavior

---

## Suggested architecture going forward

- Keep hardware ownership in one init function (`aux5::init`)
- Return the handles you need (`ITM`, `Delay`, GPIO abstractions)
- Keep logging API separate but pass references (`&mut ITM` or `&mut Stim`)
- Migrate to `defmt-rtt` when logs become frequent or structured

---

## Deferred on purpose

Per your request, this doc does **not** update:

- `README.md`
- `openocd.gdb`

Those can be aligned in a later pass once you confirm this code pattern.

