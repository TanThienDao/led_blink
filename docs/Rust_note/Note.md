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

---
### Build it

- The microcontroler in the F3 has a Cortex-M4 core, which is a powerful and efficient architecture for embedded
  systems.
    - thumbv6m-none-eabi, for the Cortex-M0 and Cortex-M1 processors
    - thumbv7m-none-eabi, for the Cortex-M3 processor
    - thumbv7em-none-eabi, for the Cortex-M4 and Cortex-M7 processors
    - thumbv7em-none-eabihf, for the Cortex-M4F and Cortex-M7F processors
- For the F3, we?ll use the thumbv7em-none-eabihf target
    - Before cross compiling, we need to add the target to our Rust toolchain:
        - `rustup target add thumbv7em-none-eabihf`
- command to build : `cargo build --target thumbv7em-none-eabihf`

---
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
                ```text
              Reading symbols from target/thumbv7em-none-eabihf/debug/led-roulette...
              Remote debugging using :3333
              0x08005200 in ?? ()
              (gdb)
                ```

            - If you see the above output, it means that GDB has successfully connected to the.
            - Then run `cargo run --target thumbv7em-none-eabihf` This is to flash ?
            - neeed to execute load.
            - ```text
                (gdb) load
                Loading section .vector_table, size 0x194 lma 0x8000000
                Loading section .text, size 0x1ea0 lma 0x8000194
                Loading section .rodata, size 0x1180 lma 0x8002034
                Start address 0x08000194, load size 12724
                Transfer rate: 17 KB/sec, 4241 bytes/write.
              ```
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
---
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
    - ``` text
      $ cargo run
      (gdb) target remote :3333
      (gdb) load
      (gdb) set print asm-demangle on
      (gdb) set style sources off
      (gdb) break main
      (gdb) continue
        ```
        - Set a breakpoint in main (or file:line), then continue:
          - ```text
            (gdb) monitor reset halt
            (gdb) break main
            (gdb) continue
            ```
        - If break main is not resolved, use line breakpoints in your file:
          - ```text
            (gdb) break src/main.rs:10
            (gdb) continue
            ```
        - Or runtime symbol fallback:
          - ```text
              (gdb) break *0x08000194
              (gdb) continue
              ```
        - Set assemble :
          - ```text
            (gdb) set print asm-demangle on
            (gdb) disassemble /m
            ```

## Release build

- `$ cargo build --target thumbv7em-none-eabihf --release` : release to the target hardware
- `cargo size --target thumbv7em-none-eabihf --bin led-blink -- -A` check up the size of the release build

## TroubleShooting

```bash
rm -f target/thumbv7em-none-eabihf/debug/led-roulette
cargo clean
cargo build --target thumbv7em-none-eabihf
file target/thumbv7em-none-eabihf/debug/led-roulette
```
---
# Print Hello Word !

- SB10 connectt to PB3
- using femal to femal jumper wire to connect SB10 to PB3
- The iprintln macro will format messages and output them to the microcontroller?s ITM.
- itm.txt file is locate at the /tmp directory, and it will be created when you run the program. You can use `tail -f /tmp/itm.txt` to view the output in real-time. This allows you to see the "Hello, world!" message printed by the microcontroller as it runs.
- OpenOCD, which is managing the debug session, can receive data sent through this ITM channel and redirect it to a file.
    - itmdump command:
        - ``` console
            $ itmdump -F -f itm.txt
            ```

## New GDB commands
- ``(gdb) c`` short command for continue
- ``(gdb) b main`` short command for break main
- ``(gdb) b src/main.rs:10`` set a breakpoint at line
- ``(gdb) b *0x08000194`` set a breakpoint at address

## Panic notet
- Ultimately, panic! is just another function call so you can see it leaves behind a trace of function calls. This allows you to use backtrace or just bt and to see call stack that caused the panic:
```aiignore 
   monitor reset halt
   delete breakpoints
   break panic_itm::panic
   continue
  ```
- If you want to see the backtrace of the panic, you can use the backtrace command in GDB after hitting the breakpoint on panic. This will show you the call stack leading up to the panic, which can help you identify where in your code the issue occurred.
---
# Register
- A register is a small amount of storage available directly on the CPU. It is used to hold data that the CPU is currently processing. Registers are much faster to access than memory, so they are used for temporary storage of data and instructions during program execution. In the context of microcontrollers, registers are used to control hardware peripherals, manage system settings, and store intermediate values during computations. Each register has a specific purpose and is accessed using specific instructions in the assembly language. Understanding how to read and manipulate registers is crucial for low-level programming and debugging of embedded systems.  
- LD3, the North LED, is connected to the pin PE9. PE9 is the short form of: Pin 9 on Port E.These pins are part of Port E so we?ll have to deal with the GPIOE peripheral.
- The table says that base address of the GPIOE register block is 0x4800_1000.
- ?BSRR? is the register which we will be using to set/reset. Its offset value is ?0x18? from the base address of the ?GPIOE?. We can look up BSRR in the reference manual. GPIO Registers -> GPIO port bit set/reset register (GPIOx_BSRR).
- `cortex_m_rt::HardFault_ (ef=0x20009fa8) at src/lib.rs:560` means that a hard fault occurred, and the error frame (ef) is located at the memory address 0x20009fa8. The source of the hard fault is indicated to be in the file src/lib.rs at line 560. A hard fault is a type of exception that occurs when the CPU encounters an unrecoverable error, such as an invalid memory access or an undefined instruction. The error frame contains information about the state of the CPU at the time of the fault, which can be useful for debugging and identifying the cause of the issue.
## Gdb commands for helpfull for debug registers
- `(gdb) list` to list the source code around the current line.
- `(gdb) print/x *ef` to print the contents of the error frame in hexadecimal format. This can help you understand the state of the CPU at the time of the hard fault, including register values and the program counter.
  - example: 
    ```asm
        $1 = cortex_m_rt::ExceptionFrame {
          r0: 0x48001800,
          r1: 0x80036b0,
          r2: 0x1,
          r3: 0x80000000,
          r12: 0xb,
          lr: 0x800020d,
          pc: 0x8001750,
          xpsr: 0xa1000200
      }
    ```
- `(gdb) disassemble /m ef.pc` to disassemble the machine code at the program counter (PC) value from the error frame. This can help you see the exact instruction that caused the hard fault, which can be crucial for diagnosing the issue.
  - eample:
    ```asm
        Dump of assembler code for function core::ptr::read_volatile<u32>:
        1046    pub unsafe fn read_volatile<T>(src: *const T) -> T {
        0x0800174c <+0>:     sub     sp, #12
        0x0800174e <+2>:     str     r0, [sp, #4]
        
        1047        if cfg!(debug_assertions) && !is_aligned_and_not_null(src) {
        1048            // Not panicking to keep codegen impact smaller.
        1049            abort();
        1050        }
        1051        // SAFETY: the caller must uphold the safety contract for `volatile_load`.
        1052        unsafe { intrinsics::volatile_load(src) }
        0x08001750 <+4>:     ldr     r0, [r0, #0]
        0x08001752 <+6>:     str     r0, [sp, #8]
        0x08001754 <+8>:     ldr     r0, [sp, #8]
        0x08001756 <+10>:    str     r0, [sp, #0]
        0x08001758 <+12>:    b.n     0x800175a <core::ptr::read_volatile<u32>+14>
        
        1053    }
        0x0800175a <+14>:    ldr     r0, [sp, #0]
        0x0800175c <+16>:    add     sp, #12
        0x0800175e <+18>:    bx      lr
        
        End of assembler dump.
    ```
    ```textmate
    How to read the assembly line by line
    
    sub sp, #12
    Reserves 12 bytes on the stack.
    
    str r0, [sp, #4]
    Saves the incoming function argument r0 to stack.
    
    ldr r0, [r0, #0]
    Loads a 32-bit value from the address in r0.
    
    This is the actual volatile read.
    str r0, [sp, #8]
    Stores the loaded value on the stack.
    
    ldr r0, [sp, #8]
    Reads it back from stack into r0.
    
    str r0, [sp, #0]
    Stores it again, probably due to compiler-generated bookkeeping.
    
    ldr r0, [sp, #0]
    Loads it into r0 for the return value.
    
    add sp, #12
    Restores the stack pointer.
    
    bx lr
    Returns to the caller.
  
    ```

    ```asm
    What each part means
    0x0800174e
    This is the memory address where this instruction lives in flash.
    <+2>
    This means ?2 bytes after the start of the current function.?
    It?s a relative offset inside the function, not a separate address.
    str
    Short for store register.
    It means: take a value from a CPU register and write it to memory.
    r0
    This is CPU register number 0.
    On ARM, r0 is often used for function arguments and return values.
    [sp, #4]
    This means: use the address in the stack pointer (sp), plus 4 bytes.
    So it stores the value into stack memory at offset 4 from sp.
    ```

## ARM/Thumb Disassembly Glossary

> Full glossary with all instructions, registers, and memory syntax is in:
> [`docs/arm-thumb-glossary.md`](docs/arm-thumb-glossary.md)

## How to read ODR: 
- ODR stands for Output Data Register. It is a register in the GPIO peripheral of a microcontroller that controls the output state of the pins. Each bit in the ODR corresponds to a specific pin, and setting a bit to 1 will drive the corresponding pin high (logic level 1), while setting it to 0 will drive the pin low (logic level 0). For example, if you want to set pin PE9 high and all other pins low, you would write the value 0x200 to the ODR. This is because 0x200 in binary is 0000 0010 0000 0000, where the bit corresponding to PE9 (bit 9) is set to 1, and all other bits are set to 0.
- How to read it for ODR:
- 0x200 = binary 0000 0010 0000 0000
- The 1 is at bit 9
- So this means: PE9 = high, all other PE bits = low (assuming only lower 16 bits matter)
- Note: 
  - Another calculate is 2^9 = 512, which is 0x200 in hexadecimal. This confirms that setting bit 9 corresponds to the value 0x200 in the ODR.
  - binary 0000 0010 0000 0000 is actually 1 on the 9 bit, which means that PE9 is set to high (logic level 1), while all other bits (and thus all other pins) are set to low (logic level 0).













