# LED Blink — STM32F3 Discovery (Embedded Rust)

A bare-metal Rust program for the **STM32F3 Discovery** board that runs two alternating LED animations:

1. **Blink** — all 8 user LEDs flash on/off together
2. **Roulette** — a single LED chases around the 8-LED ring
   <img src="led_demo.gif" width="300" alt="LED Demo" style="transform: rotate(270deg);" />

---

## Demo Behaviour

```
┌─ Phase 1: Blink (×5) ──────────────────────────────┐
│  ALL 8 LEDs ON  →  wait 200 ms  →  ALL 8 LEDs OFF  │
│  repeat 5 times                                      │
└─────────────────────────────────────────────────────┘
           ↓
┌─ Phase 2: Roulette (×8 steps) ──────────────────────┐
│  LED[0] → LED[1] → LED[2] → … → LED[7] → (repeat)  │
│  each step: next ON, wait 50 ms, current OFF         │
└─────────────────────────────────────────────────────┘
           ↓  loop forever
```

---

## Source Code Overview

### `src/main.rs`

```rust
const BLINK_PERIOD_MS: u16 = 200;   // all-LEDs blink delay
const ROULETTE_PERIOD_MS: u16 = 50; // roulette step delay
const BLINK_CYCLES: u8 = 5;         // full on/off blinks before roulette
```

Three constants at the top let you tune timing without touching loop logic.

**Main loop:**

```rust
loop {
    // Phase 1 — all 8 LEDs blink on/off together
    for _ in 0..BLINK_CYCLES {
        all_on(&mut leds);
        delay.delay_ms(BLINK_PERIOD_MS);
        all_off(&mut leds);
        delay.delay_ms(BLINK_PERIOD_MS);
    }

    // Phase 2 — one LED chases around the ring
    for curr in 0..leds.len() {
        let next = (curr + 1) % leds.len();
        leds[next].on().ok();
        delay.delay_ms(ROULETTE_PERIOD_MS);
        leds[curr].off().ok();
    }
}
```

**Helper functions:**

- `all_on(leds)` — iterates the LED array and turns every LED on
- `all_off(leds)` — iterates the LED array and turns every LED off

### `auxiliary/src/lib.rs` (`aux5` crate)

Local support crate that wraps board initialisation:

- Configures system clocks via RCC
- Initialises GPIO port E (PE8–PE15) as push-pull outputs
- Returns `(Delay, LedArray)` ready to use

---

## Hardware

| Item        | Detail                     |
|-------------|----------------------------|
| Board       | STM32F3 Discovery          |
| MCU         | STM32F303VCT6 (Cortex-M4F) |
| LEDs        | PE8–PE15 — 8 × user LEDs   |
| Debug probe | ST-LINK/V2.1 (on-board)    |
| Target      | `thumbv7em-none-eabihf`    |

---

## Prerequisites

```bash
# Cross-compilation target
rustup target add thumbv7em-none-eabihf

# GDB and OpenOCD
sudo apt install gdb-multiarch openocd
```

### udev rule (flash without `sudo`)

Create `/etc/udev/rules.d/99-openocd.rules`:

```
SUBSYSTEM=="usb", ATTR{idVendor}=="0483", ATTR{idProduct}=="374b", MODE="0666", GROUP="plugdev"
```

```bash
sudo udevadm control --reload-rules && sudo udevadm trigger
```

---

## Build

```bash
cargo build
```

---

## Flash & Run

**Terminal 1** — start OpenOCD (keep running):

```bash
openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
```

**Terminal 2** — flash and attach GDB:

```bash
cargo run
```

`cargo run` invokes `gdb-multiarch -q -x openocd.gdb` which automatically:

1. Connects to OpenOCD (`:3333`)
2. Flashes firmware (`load`)
3. Sets breakpoints at `main`, `DefaultHandler`, `HardFault`
4. Breaks at `main` entry, then steps into `main` body

At the GDB prompt, run:

```
(gdb) continue
```

The LEDs will start animating on the board.

---

## Release Build

```bash
cargo build --release

# Inspect binary sections and sizes
cargo size --target thumbv7em-none-eabihf --bin led-blink -- -A
```

---

## Project Structure

```
led-blink/
├── .cargo/
│   └── config.toml        # Default target + cargo run runner
├── auxiliary/
│   └── src/lib.rs         # aux5: clocks, delay, LED array init
├── src/
│   └── main.rs            # Blink + roulette animation loop
├── openocd.gdb            # GDB script: connect, flash, break at main
└── Cargo.toml
```

---

## Troubleshooting

| Symptom                                  | Cause                                           | Fix                                                                     |
|------------------------------------------|-------------------------------------------------|-------------------------------------------------------------------------|
| `openocd.gdb: No such file or directory` | Running from wrong directory                    | `cd` to project root before `cargo run`                                 |
| `Cannot configure TPIU/SWO`              | Deprecated `monitor tpiu config` in `.gdb` file | Remove those lines from `openocd.gdb`                                   |
| `is not a core dump`                     | Binary path passed twice via `--` args          | Run plain `cargo run` with no extra args                                |
| No OpenOCD connection                    | OpenOCD not started                             | Run `openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg` first |

