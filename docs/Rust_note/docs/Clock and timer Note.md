# For loop delays

The first challenge is to implement the delay function without using any peripheral and the obvious solution is to implement it as a for loop delay:

```rust
#[inline(never)]
fn delay(tim6: &tim6::RegisterBlock, ms: u16) {
    for _ in 0..1_000 {}
}
```

Of course, the above implementation is wrong because it always generates the same delay for any value of ms.

In this section, you’ll have to:

- Fix the delay function to generate delays proportional to its input ms.
- Tweak the delay function to make the LED roulette spin at a rate of approximately 5 cycles in 4 seconds (800 milliseconds period).
- The processor inside the microcontroller is clocked at 72 MHz and executes most instructions in one “tick”, a cycle of its clock. How many (for) loops do you think the delay function must do to generate a delay of 1 second?
- How many for loops does delay(1000) actually do?
- What happens if compile your program in release mode and run it?

* * *

### 1. Fix the `delay` function

To make the delay proportional to the input `ms`, you need to multiply `ms` by a constant factor in the loop range. Since `ms` is a `u16`, it is safer to cast it to `u32` to avoid overflow if the multiplier is large.

The problem with the original code is that it always counts to 1,000, no matter what number you pass in. You need to multiply the ms input by a "Magic Number" (<span>K</span>) to make the loop last longer when ms is higher.

```rust
    #[inline(never)]  
    fn delay(_tim6: &tim6::RegisterBlock, ms: u16) {  
        // We use a multiplier (K) to adjust the duration.  
        // For now, let's use a placeholder K.  
        for _ in 0..(ms as u32 * 12_000) {  
            // We can add a NOP to prevent the compiler from optimizing  
            // the loop away entirely in some cases, though inline(never)  
            // and the empty loop are the focus here.  
            cortex_m::asm::nop();  
        }  
    }  
```

* * *

## Step 2: Doing the math (The 1-second delay)  

The exercise asks: **"How many loops do you think it needs for 1 second?"

1. **Understand the Clock**: Your STM32 runs at **72 MHz**. This means it can perform **72,000,000 "ticks"** per second.  
2. **Understand the Loop**: A `for` loop isn't just one instruction. The CPU has to:  
    * Add 1 to the counter.  
    * Check if the counter is at the limit.  
    * Jump back to the start.  
3. **The Guess**: If we assume these 3-4 steps take **4 clock cycles** total:  
    * $72,000,000  { cycles} / 4 = {18,000,000 { loops}}$.  
4. **The 1ms Multiplier**: To get 1 millisecond, divide that by 1,000:  
    * $18,000,000 / 1,000 = {18,000}$.  
    * So, if you use `ms as u32 * 18_000`, you are roughly at 1ms per `ms`.

&nbsp;

Hertz to Megahertz (Conversion)

- **Hz (Hertz):** 1 cycle per second.
- **kHz (Kilohertz):** 1,000 cycles per second.
- **MHz (Megahertz):** 1,000,000 cycles per second.
- **GHz (Gigahertz):** 1,000,000,000 cycles per second. [[1](https://www.corsair.com/us/en/explorer/diy-builder/memory/mts-vs-mhz-explained/), [2](https://www.lenovo.com/ph/en/glossary/what-is-megahertz/), [3](https://www.pcmag.com/encyclopedia/term/mhz), [4](https://www.ubergizmo.com/what-is/mhz/)]



* * *

### Step 3: Tweak for the "5 cycles in 4 seconds"  
The goal is to make the LED light spin around the circle 5 times in 4 seconds.  
* **Total Time**: 4,000 milliseconds.  
* **Rotations**: 5.  
* **Time per Rotation**: $4,000 / 5 = {800 { ms}}$.

Since your `main.rs` has **8 LEDs**, and the loop switches an LED **twice** (once on, once off), you are calling `delay` **16 times** per full rotation.  
* $800 {ms} / 16 {delays} = {50 { ms}}$.  
* **The Tweak**: If your code is set to `let ms = 50;`, you just need to adjust your "Magic Number" from Step 1 until the circle spin takes exactly 0.8 seconds.

### How many loops does `delay(1000)` actually do?

- In the **original** broken code (`for _ in 0..1_000 {}`), it always performs exactly **1,000** loops, regardless of the `ms` input.
- In your **fixed** code (`for _ in 0..(ms as u32 * K) {}`), calling `delay(1000)` will perform $1000 times K$ loops.

* * *

### Step 4: The "Release Mode" Mystery  
The exercise asks what happens in **Release Mode** (`--release`).

* **What happens**: The LEDs will spin so fast they all look like they are on at once, or it just looks "wrong."  
* **Why?**: Compilers are designed to be "smart." In Release Mode, the compiler looks at your code and says: *"Wait, this loop just counts to 1,000,000 and does absolutely nothing with the result. That's a waste of time! I'll just delete it."*  
* **The Result**: The `delay` function becomes empty, and the time it takes to run drops to almost **zero**.

**How to prevent this?**
Use `cortex_m::asm::nop()` inside the loop or wrap the loop counter in `core::hint::black_box` to tell the compiler that the loop performs "work" it shouldn't optimize away. However, the best practice in embedded development is to use a hardware timer (like `TIM6` which is passed into the function) instead of a busy-wait loop.

### Summary for your notes:  
1. **Calculations**: Based on the **72MHz** CPU speed.  
2. **Implementation**: Use a multiplier like `ms * 10_000` (this is a guess, you have to test it).  
3. **Warning**: For-loop delays are "bad" because the compiler tries to delete them and they stop being accurate if the CPU speed changes. That's why the next chapter will teach you to use **Hardware Timers** (`TIM6`) instead!

* * *
# Understanding TIM6 Registers in STM32
In the STM32F303 (and similar STM32 microcontrollers), **TIM6** is a basic 16-bit timer. It is often used for simple time bases or to trigger the DAC.

The registers you mentioned?`PSC`, `CNT`, `ARR`, `SR`, and `EGR`?work together to control how the timer counts and when it signals an "event" (like a timeout).

### 1. `PSC` - Prescaler Register
The **Prescaler** determines how fast the counter (`CNT`) increments.
*   **Function:** It divides the incoming clock frequency (usually from the APB1 bus) by a factor of `(PSC + 1)`.
*   **Formula:** $f_{CNT} = \frac{f_{CK\_PSC}}{PSC + 1}$
*   **Example:** If your clock is 8 MHz and you want the counter to increment every 1 millisecond (1 kHz), you set `PSC` to `7999`.
    *   $8,000,000 / (7999 + 1) = 1,000$ Hz (1 tick per ms).

### 2. `CNT` - Counter Register
The **Counter** is the heart of the timer.
*   **Function:** This is a 16-bit register that holds the current count value.
*   **Behavior:** When the timer is enabled (`CEN` bit in `CR1`), `CNT` increments on every clock pulse coming from the prescaler. It counts from `0` up to the value in `ARR`.

### 3. `ARR` - Auto-Reload Register
The **Auto-Reload** register defines the "target" or "ceiling" for the counter.
*   **Function:** It holds the value that `CNT` must reach to trigger an **Update Event**.
*   **One-pulse mode:** In the context of your project ([one-shot-timer.md](/home/tandao/Documents/repos/rust_idemy/embedded_rust/discovery/f3discovery/src/09-clocks-and-timers/one-shot-timer.md)), when `CNT` reaches `ARR`, the timer stops and sets a flag.
*   **Continuous mode:** When `CNT` reaches `ARR`, it resets to `0` and starts counting again immediately.

### 4. `SR` - Status Register
The **Status Register** is used to check the state of the timer.
*   **Key Bit (`UIF`):** The **Update Interrupt Flag**. This bit is set by hardware whenever an update event occurs (e.g., when the counter reaches the `ARR` value).
*   **Usage:** In a "busy-wait" delay, you monitor this bit:
    ```rust
    // Wait until the UIF bit is set (meaning time is up)
    while !tim6.sr.read().uif().bit_is_set() {}
    // Clear the flag manually after reading
    tim6.sr.modify(|_, w| w.uif().clear_bit());
    ```

### 5. `EGR` - Event Generation Register
The **Event Generation Register** is a write-only register used to trigger events via software.
*   **Key Bit (`UG`):** The **Update Generation** bit.
*   **Function:** Writing a `1` to this bit forces an "Update Event" immediately.
*   **Why use it?** When you change the `PSC` or `ARR` values, the changes might not take effect until the *next* update event. Writing to `EGR.UG` forces the hardware to reload the prescaler and reset the counter to `0` immediately, ensuring your new settings are applied right away.

### Summary of the Cycle
1.  **Clock** enters the **PSC** and gets slowed down.
2.  The slowed-down clock makes **CNT** tick upward.
3.  When **CNT** hits the value in **ARR**, an **Update Event** happens.
4.  The **SR.UIF** flag is set to `1` (which you can check in your code).
5.  If you want to restart or apply new settings mid-count, you use **EGR.UG**.