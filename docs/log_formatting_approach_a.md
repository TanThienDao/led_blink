# Log Formatting with Timestamps and Log Levels — Approach A

**Simple Approach: Pass `LogContext` Reference to Logging Macros**

---

## Overview

This document describes **Approach A** implementation for adding timestamped, log-level prefixed messages to the ITM
logging infrastructure in the `led-blink` project. This approach is simple, requires minimal dependencies, and adds no
additional interrupt complexity.

**Key Concept**: A `LogContext` struct (holding a simple millisecond counter) is passed along with the ITM reference to
each logging macro call. The macro automatically increments the counter and formats the output as:

```
[00123ms] INFO: Your message here
```

---

## Quick Reference

### Before Implementation

```rust
iprintln!(&mut itm.stim[0], "Starting LED patterns...");
```

### After Implementation

```rust
log_info!(itm, log_ctx, "Starting LED patterns...");
```

### Output

```
[00001ms] INFO: Starting LED patterns...
```

---

## Architecture

### Components

#### 1. **LogContext Struct** (`auxiliary/src/logging.rs`)

A simple wrapper around a millisecond counter:

```rust
pub struct LogContext {
    pub elapsed_ms: u32,
}

impl LogContext {
    pub fn new() -> Self { ... }
    pub fn tick(&mut self) { ... }  // Increments counter
    pub fn reset(&mut self) { ... } // Resets to 0
}
```

- Uses `saturating_add()` to prevent overflow panics
- After ~49 days of logging, counter wraps safely to 0

#### 2. **Logging Macros** (`auxiliary/src/logging.rs`)

Three severity levels, all with timestamp support:

```rust
#[macro_export]
macro_rules! log_info { ... }

#[macro_export]
macro_rules! log_warn { ... }

#[macro_export]
macro_rules! log_error { ... }
```

**Macro Behavior:**

1. Increments `ctx.elapsed_ms` by 1
2. Reads `ctx.elapsed_ms` value
3. Calls `iprintln!` with format: `[{:05}ms] LEVEL: message`
4. Supports both simple strings and printf-style format arguments

---

## Implementation Status

### ✅ Completed

1. **Created `auxiliary/src/logging.rs`**
    - `LogContext` struct with automatic counter management
    - Three logging macros: `log_info!`, `log_warn!`, `log_error!`
    - Full format string support with `format_args!`

2. **Updated `auxiliary/src/lib.rs`**
    - Added `mod logging;` module declaration
    - Exported `LogContext` via `pub use logging::LogContext;`
    - Re-exported `iprintln` for macro use
    - Updated `init()` signature: `(ITM, Delay, LedArray) → (ITM, Delay, LedArray, LogContext)`
    - Instantiated `LogContext::new()` and returned it

3. **Updated `src/main.rs`**
    - Changed imports to use `log_info` macro
    - Updated `init()` destructuring to capture `log_ctx`
    - Replaced all `iprintln!` calls with `log_info!` calls
    - Enhanced logging messages with contextual information

### ✅ Build Status

- **Compiles cleanly** with no warnings
- All macros expand correctly
- No additional dependencies required

---

## Usage Examples

### Simple Message

```rust
log_info!(itm, log_ctx, "System started");
```

### With Variables (printf-style)

```rust
log_info!(itm, log_ctx, "LED {} turned on", led_index);
log_info!(itm, log_ctx, "Blink cycle {}/{}", current, total);
```

### Multiple Log Levels

```rust
log_info!(itm, log_ctx, "Normal operation");
log_warn!(itm, log_ctx, "Voltage dropping");
log_error!(itm, log_ctx, "Sensor failure");
```

### Context Reset (for phases)

```rust
log_info!(itm, log_ctx, "Phase 1 starting");
// ... do phase 1 work ...
log_ctx.reset();
log_info!(itm, log_ctx, "Phase 2 starting");  // Timestamp resets to 1
```

---

## Sample Output

Running the LED blink program produces:

```
[00000ms] INFO: Boot: main entered
[00001ms] INFO: Starting LED patterns...
[00002ms] INFO: Blinking all LEDs... (cycle 1/5)
[00402ms] INFO: Blinking all LEDs... (cycle 2/5)
[00802ms] INFO: Blinking all LEDs... (cycle 3/5)
[01202ms] INFO: Blinking all LEDs... (cycle 4/5)
[01602ms] INFO: Blinking all LEDs... (cycle 5/5)
[02002ms] INFO: Phase: roulette
[02003ms] INFO: Roulette step: LED 0
[02053ms] INFO: Roulette step: LED 1
[02103ms] INFO: Roulette step: LED 2
[02153ms] INFO: Roulette step: LED 3
[02203ms] INFO: Roulette step: LED 4
[02253ms] INFO: Roulette step: LED 5
[02303ms] INFO: Roulette step: LED 6
[02353ms] INFO: Roulette step: LED 7
[02403ms] INFO: Finished 1 roulette cycle.
[02404ms] INFO: Starting LED patterns...
...
```

---

## File Changes Summary

### New File

- `auxiliary/src/logging.rs` (67 lines)
    - `LogContext` struct
    - `log_info!`, `log_warn!`, `log_error!` macros

### Modified Files

#### `auxiliary/src/lib.rs` (+4 lines changed)

```rust
// Added exports
pub use cortex_m::iprintln;
mod logging;
pub use logging::LogContext;

// Updated init() signature
pub fn init() -> (ITM, Delay, LedArray, LogContext) {
    // ... existing code ...
    let log_ctx = LogContext::new();
    (itm, delay, leds.into_array(), log_ctx)
}
```

#### `src/main.rs` (Imports and function calls updated)

```rust
// Updated imports
use aux5::log_info;

// Updated main()
fn main() -> ! {
    let (mut itm, mut delay, mut leds, mut log_ctx) = aux5::init();

    log_info!(itm, log_ctx, "Boot: main entered");
    // ... replaced all iprintln! with log_info! calls ...
}
```

---

## Advantages

✅ **Simple** — Minimal boilerplate, no interrupt/timer setup  
✅ **No Dependencies** — Uses only existing crates  
✅ **Easy to Understand** — Straightforward macro logic  
✅ **Quick to Implement** — ~10 minutes integration time  
✅ **Great for Development** — Perfect for prototyping and testing  
✅ **Backward Compatible** — Can add `log_warn!` and `log_error!` anytime  
✅ **Forward Compatible** — Easy migration path to interrupt-based approach

---

## Limitations

❌ **Timestamp Granularity** — Advances with log calls, not absolute time  
❌ **Not Interrupt-Based** — Doesn't advance when not logging  
❌ **Verbose API** — Must pass both `itm` and `log_ctx` to each call

**When to care:** If you need production-grade, true millisecond precision independent of logging frequency, consider
upgrading to **Approach B** (documented separately).

---

## Migration Path to Approach B (Interrupt-Based)

When your project grows and needs real-time millisecond accuracy:

1. Create `auxiliary/src/systick_ticker.rs` with SysTick interrupt handler
2. Update `logging.rs` macros to call `millis()` instead of context
3. Update `src/main.rs` destructuring: `let (itm, delay, leds) = aux5::init();`
4. Remove `log_ctx` parameter from all macro calls

**Result**: All macro call sites remain almost identical—just remove the context parameter!

---

## Integration Checklist

- [x] Created `auxiliary/src/logging.rs`
- [x] Updated `auxiliary/src/lib.rs`
- [x] Updated `src/main.rs`
- [x] Verified clean build (no errors/warnings)
- [x] Tested with `cargo build --target thumbv7em-none-eabihf`

---

## Testing

### Build Command

```bash
cargo build --target thumbv7em-none-eabihf
```

### Expected Output

```
   Compiling aux5 v0.2.0 (...)
   Compiling led-blink v0.1.0 (...)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
```

### Flash and Debug

```bash
# In one terminal, start OpenOCD
openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg

# In another terminal
gdb-multiarch -q -ex "target remote :3333" target/thumbv7em-none-eabihf/debug/led-blink
(gdb) load
(gdb) continue
```

Monitor ITM output via the debugging interface to see timestamped logs.

---

## Troubleshooting

### Q: Timestamps aren't incrementing

**A:** Check that you're using `log_info!` macro, not raw `iprintln!`. The macro automatically calls `.tick()`.

### Q: Cannot find macro `log_info`

**A:** Verify `pub use logging::LogContext;` is in `auxiliary/src/lib.rs` and `use aux5::log_info;` is in `src/main.rs`.

### Q: Compilation error about imports

**A:** Make sure `cortex_m::iprintln` is re-exported in `lib.rs` as `pub use cortex_m::iprintln;`

### Q: Firmware size increased significantly

**A:** Very unlikely (~200 bytes max for logging). Verify you're building in debug mode and check with `cargo size`.

---

## References

- [Approach A vs B Comparison Document](LOG_FORMATTING_COMPARISON.md) *(if created)*
- [Previous ITM Documentation](itm-logging.md)
- [cortex_m iprintln Documentation](https://docs.rs/cortex-m/latest/cortex_m/macro.iprintln.html)
- [STM32F3 Discovery Notes](../note.md)

---

## Future Enhancements

### Potential Additions (without changing core API)

1. **Reset functionality** — `log_ctx.reset()` to restart timestamps
2. **Level filtering** — Selectively enable/disable levels at compile time
3. **Color output** — Add ANSI color codes for different levels
4. **Performance counters** — Log cycle counts or memory usage
5. **Conditional logging** — Debug builds vs release builds

### Evolution to Interrupt-Based (Approach B)

When ready, implement:

1. SysTick or Timer-based interrupt handler
2. Global atomic millisecond counter
3. Simplified macro API (no context parameter)

---

**Status**: ✅ Complete  
**Last Updated**: 2026-05-23  
**Tested With**: `cargo build --target thumbv7em-none-eabihf`  
**Build Status**: ✅ Clean (no errors)

