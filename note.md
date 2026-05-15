### Note for STM32 F3 Discover

- the STM32 F4 USB was
    - `lsusb | grep ST-LINK`
    - `Bus 001 Device 003: ID 0483:374b STMicroelectronics ST-LINK/V2.1`
    - planning to create the `/etc/udev/rules.d/*.rules`
    - this rule allows the user to use the hardware without `sudo`.
- STM32F5 info was:
    - `idVendor = 0483` (STMicroelectronics)
    - `idProduct = 374b` (ST-LINK/V2.1)
- the udev rule is (Create /etc/udev/rules.d/99-openocd.rules):
    - `SUBSYSTEM=="usb", ATTR{idVendor}=="0483", ATTR{idProduct}=="374b", MODE="0666", GROUP="plugdev"`
- after creating the udev rule, reload the rules with:
    - `sudo udevadm control --reload-rules`
    - `sudo udevadm trigger`

- The Motion Sensors (The "MEMS")
    - The board contains two main chips that provide "9-axis" sensing capability.
    - They communicate with the MCU via the **I2C** protocol.

  | Component     | Function         | What it Measures                                                                                      |
      |---------------|------------------|-------------------------------------------------------------------------------------------------------|
  | Accelerometer | Linear Motion    | Measures acceleration (including gravity) in X, Y, and Z axes. Used to detect tilt or movement.      |
  | Magnetometer  | Magnetic Field   | Measures the Earth's magnetic field. Allows the board to function as a digital compass.               |
  | Gyroscope     | Angular Velocity | Measures how fast the board is rotating around its axes. Vital for stabilizing drones or tracking.   |

### Build it

- The microcontroler in the F3 has a Cortex-M4 core, which is a powerful and efficient architecture for embedded
  systems.
    - thumbv6m-none-eabi, for the Cortex-M0 and Cortex-M1 processors
    - thumbv7m-none-eabi, for the Cortex-M3 processor
    - thumbv7em-none-eabi, for the Cortex-M4 and Cortex-M7 processors
    - thumbv7em-none-eabihf, for the Cortex-M4F and Cortex-M7F processors
- For the F3, we’ll use the thumbv7em-none-eabihf target
    - Before cross compiling, we need to add the target to our Rust toolchain:
        - `rustup target add thumbv7em-none-eabihf`
- command to build : `cargo build --target thumbv7em-none-eabihf`

## Flash it

- Flashing is the process of writing the compiled program (firmware) onto the microcontroller's memory. This allows the
  microcontroller to execute the program when powered on.
    - `cd /tmp`
        - `openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg`
            - We use the command openocd.
                - openocd is an open-source tool that provides debugging, in-system programming, and boundary-scan
                  testing for embedded devices. It supports a wide range of microcontrollers and development boards,
                  including the STM32 F3 Discovery.
                - Stand for "Open On-Chip Debugger", OpenOCD allows developers to interact with the hardware directly,
                  enabling tasks such as flashing firmware, setting breakpoints, and inspecting memory.
                - `-f interface/stlink-v2-1.cfg` load config for debug adapter (ST-LINK/V2.1).
                - `-f target/stm32f3x.cfg` load config for target MCU family (STM32F3x).
            - Starts OpenOCD.
                - Breakdown:
                - openocd = Open On-Chip Debugger
                - `-f interface/stlink-v2-1.cfg` = load the config for the ST-LINK debug adapter on the board
                - `-f target/stm32f3x.cfg` = load the config for the STM32F3 target MCU
            - Execute GDB section
                - `arm-none-eabi-gdb -q -ex "target remote :3333" target/thumbv7em-none-eabihf/debug/led-roulette`
                - `gdb-multiarch -q -ex "target remote :3333" target/thumbv7em-none-eabihf/debug/led-roulette`
                - `gdb -q -ex "target remote :3333" target/thumbv7em-none-eabihf/debug/led-roulette`
            - sucessfull case:
                ```
              Reading symbols from target/thumbv7em-none-eabihf/debug/led-roulette...
              Remote debugging using :3333
              0x08005200 in ?? ()
              (gdb)
                ```

            - If you see the above output, it means that GDB has successfully connected to the.
            - Then run `cargo run --target thumbv7em-none-eabihf` This is to flash ?
            - neeed to execute load.
            - ```
                (gdb) load
                Loading section .vector_table, size 0x194 lma 0x8000000
                Loading section .text, size 0x1ea0 lma 0x8000194
                Loading section .rodata, size 0x1180 lma 0x8002034
                Start address 0x08000194, load size 12724
                Transfer rate: 17 KB/sec, 4241 bytes/write.```
    - After loading, you can run the program with `continue` or `c` in GDB.
        - `(gdb) continue`
          -(gdb) load means: GDB tells OpenOCD to program your ELF into the MCU flash memory.
    - Line by line:
        - <span style="color: #FFD700;">Loading section .vector_table, size 0x194 lma 0x8000000</span>
            - Programs the interrupt vector table.
            - size 0x194 = 0x194 bytes (404 bytes).
            - lma 0x08000000 = flash address where this section is written.
        - <span style="color: #FFD700;">Loading section .text, size 0x1ea0 lma 0x8000194</span>
            - Programs executable machine code (your functions/instructions).
            - size 0x1ea0 = 7840 bytes.
            - Starts right after vector table at 0x08000194.
        - <span style="color: #FFD700;">Loading section .rodata, size 0x1180 lma 0x8002034</span>
            - Programs read-only constants (e.g., string literals, const tables).
            - size 0x1180 = 4480 bytes.
            - Written at 0x08002034.
        - <span style="color: #FFD700;">Start address 0x08000194, load size 12724</span>
            - Entry/start PC from the ELF is 0x08000194 (where execution begins after reset).
            - The entry point is the part of a program that a processor CPU eill execute first.
            - Total bytes programmed this time: 12,724 bytes.
        - <span style="color: #FFD700;">Transfer rate: 17 KB/sec, 4241 bytes/write.
            - Effective flashing speed.
            - 4241 bytes/write is the average chunk size sent per flash write operation.

## Debug it

- Debugging is the process of identifying and fixing issues in your code. When working with embedded
    - `disassemble /m` command in GDB is used to display the disassembled machine code of the program. This allows you
      to see the low-level instructions that the CPU will execute, which can be helpful for understanding how your
      high-level code translates into machine code and for diagnosing issues at the instruction level.
    - `load` command in GDB is used to load the compiled program (firmware) into the target device's memory. This allows
      you to run and debug the program on the actual hardware.
    - `continue` or `c` command in GDB is used to start or resume the execution of the program on the target device
      after it has been loaded. This allows you to see how the program behaves in real-time and helps you identify any
      issues or bugs that may be present.
    - `break main` or `b main` command in GDB is used to set a breakpoint at the beginning of the `main` function. This
      allows you to pause the execution of the program when it reaches the `main` function, giving you the opportunity
      to inspect variables, check the call stack, and analyze the program's behavior at that point.
    - `step` or `s` command in GDB is used to execute the next line of code in the program. If the next line is a
      function call, it will step into
    - `print variable_name` or `p variable_name` command in GDB is used to display the value of a specific variable.
      This allows you to check the state of variables at different points in the program's execution, which can help you
      understand how the program is functioning and identify any issues.
    - `info locals` command in GDB is used to display the values of all local variables in the current function. This
      provides a snapshot of the local state of the program, which can be useful for debugging and understanding how the
      function is operating.
    - `set print asm-demangle on` command in GDB is used to enable the demangling of assembly symbols. This means that
      when you view disassembled code or backtraces, GDB will attempt to convert mangled symbol names (which are often
      generated by C++ compilers) into more human-readable forms. This can make it easier to understand the code and
      identify functions and variables when debugging.
    - `monitor reset halt` command in OpenOCD is used to reset the target device and halt its execution. This allows you
      to start debugging from a known state, ensuring that the program is not running and that you can set breakpoints
      or inspect memory before execution begins.
    - `layout src` command in GDB is used to switch the display layout to show the source code. This allows you to see
      the original source code alongside the assembly instructions, making it easier to understand how the high-level
      code corresponds to the low-level machine code during debugging.
    - `layout adm` command in GDB is used to switch the display layout to show the assembly code. This allows you to
      focus on the disassembled machine code, which can be helpful for low-level debugging and understanding how the CPU
      executes instructions.
    - `layout split` command in GDB is used to switch the display layout to a split view, showing both the source code
      and the assembly code simultaneously. This allows you to see how the high-level source code corresponds to the
      low-level assembly instructions, providing a comprehensive view of the program's execution during debugging.
    - ``` aiignore
      $ cargo run
      (gdb) target remote :3333
      (gdb) load
      (gdb) set print asm-demangle on
      (gdb) set style sources off
      (gdb) break main
      (gdb) continue
        ```
        - Set a breakpoint in main (or file:line), then continue:
        - ```aiignore
          (gdb) monitor reset halt
          (gdb) break main
          (gdb) continue
          ```
        - If break main is not resolved, use line breakpoints in your file:
        - ```aiignore
          (gdb) break src/main.rs:10
          (gdb) continue
            ```
        - Or runtime symbol fallback:
            - ```aiignore
           (gdb) break *0x08000194
           (gdb) continue
           ```
        - Set assemble :
        - ```aiignore
           gdb) set print asm-demangle on
           (gdb) disassemble /m
           ```

## Release build

- `$ cargo build --target thumbv7em-none-eabihf --release` : release to the target hardware
- `cargo size --target thumbv7em-none-eabihf --bin led-blink -- -A` check up the size of the release build

## TroubleShooting

```aiignore
rm -f target/thumbv7em-none-eabihf/debug/led-roulette
cargo clean
cargo build --target thumbv7em-none-eabihf
file target/thumbv7em-none-eabihf/debug/led-roulette
```