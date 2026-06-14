# ARM/Thumb Disassembly Glossary

A beginner-friendly reference for reading ARM/Thumb assembly instructions,  
as seen in GDB disassembly output when debugging embedded Rust on the STM32F3.

* * *

## How to read a disassembly line

```asm
0x0800174e <+2>:    str     r0, [sp, #4]
```

| Part | What it means |
| --- | --- |
| `0x0800174e` | Memory address where this instruction lives in flash |
| `<+2>` | Offset in bytes from the start of the current function |
| `str` | The instruction opcode (the operation to perform) |
| `r0` | Source register (operand) |
| `[sp, #4]` | Destination memory address = stack pointer + 4 bytes |

* * *

## Core Instructions

| Mnemonic | Full name | What it does | Example |
| --- | --- | --- | --- |
| `ldr` | Load Register | Reads a value **from memory** into a register | `ldr r0, [r0, #0]` |
| `str` | Store Register | Writes a value **from a register** into memory | `str r0, [sp, #4]` |
| `add` | Add | Adds two values; often adjusts stack or addresses | `add sp, #12` |
| `sub` | Subtract | Subtracts; often reserves stack space | `sub sp, #12` |
| `mov` | Move | Copies a value from one register/constant to another | `mov r0, #0` |
| `cmp` | Compare | Compares two values and sets CPU flags (no result saved) | `cmp r0, #0` |
| `push` | Push registers | Saves registers onto the stack | `push {r0, lr}` |
| `pop` | Pop registers | Restores registers from the stack | `pop {r0, pc}` |
| `nop` | No Operation | Does nothing; used for timing/alignment | `nop` |
| `and` | Bitwise AND | ANDs two values together | `and r0, r1` |
| `orr` | Bitwise OR | ORs two values together | `orr r0, r1` |
| `eor` | Bitwise XOR | XORs two values together | `eor r0, r1` |
| `lsl` | Logical Shift Left | Shifts bits left (multiply by 2 per shift) | `lsl r0, #1` |
| `lsr` | Logical Shift Right | Shifts bits right (divide by 2 per shift) | `lsr r0, #1` |

* * *

## Branch Instructions

Branches are "jump" instructions — they tell the CPU to go to a different address.

| Mnemonic | Full name | What it does | Example |
| --- | --- | --- | --- |
| `b` | Branch | Unconditional jump to an address | `b 0x8001760` |
| `b.n` | Branch, Narrow | Short 16-bit Thumb branch (same as `b` but compact) | `b.n 0x800175a` |
| `bl` | Branch with Link | Calls a function; saves return address into `lr` | `bl some_function` |
| `bx` | Branch and Exchange | Jumps to address in a register; usually used to return | `bx lr` |
| `blx` | Branch with Link and Exchange | Calls a function via register | `blx r3` |
| `b.eq` | Branch if Equal | Jumps only if previous `cmp` was equal | `b.eq 0x8001780` |
| `b.ne` | Branch if Not Equal | Jumps only if previous `cmp` was not equal | `b.ne 0x8001780` |
| `b.lt` | Branch if Less Than | Jumps if signed less than | `b.lt 0x8001780` |
| `b.gt` | Branch if Greater Than | Jumps if signed greater than | `b.gt 0x8001780` |
| `b.le` | Branch if Less or Equal | Jumps if signed less than or equal | `b.le 0x8001780` |
| `b.ge` | Branch if Greater or Equal | Jumps if signed greater than or equal | `b.ge 0x8001780` |
| `b.cs` | Branch if Carry Set | Jumps if carry flag is set (unsigned overflow) | `b.cs 0x8001780` |
| `b.cc` | Branch if Carry Clear | Jumps if carry flag is clear | `b.cc 0x8001780` |

* * *

## Special Registers

| Register | Full name | What it holds |
| --- | --- | --- |
| `r0` | Register 0 | First function argument; also holds return values |
| `r1` | Register 1 | Second function argument |
| `r2` | Register 2 | Third function argument |
| `r3` | Register 3 | Fourth function argument |
| `r4`–`r11` | General purpose | Caller-saved working registers |
| `r12` | Intra-procedure Call Register | Temporary register used by the linker/caller |
| `sp` | Stack Pointer (`r13`) | Points to the current top of the stack |
| `lr` | Link Register (`r14`) | Holds the return address after a function call |
| `pc` | Program Counter (`r15`) | Holds the address of the current instruction |
| `xpsr` | Program Status Register | CPU flags: zero, carry, overflow, negative, etc. |

* * *

## Memory Addressing Syntax

| Syntax | Meaning | Example |
| --- | --- | --- |
| `[r0]` | Memory at address in `r0` | `ldr r1, [r0]` |
| `[r0, #4]` | Memory at address `r0 + 4` | `ldr r1, [r0, #4]` |
| `[sp, #8]` | Memory at address `sp + 8` | `str r0, [sp, #8]` |
| `[r0, #0]` | Memory at exact address in `r0` (offset 0) | `ldr r0, [r0, #0]` |
| `#12` | The number 12 directly (immediate value) | `sub sp, #12` |

> **Rule:** Brackets `[ ]` always mean "go to that memory address".  
> Without brackets, you're using the value directly.

* * *

## Common Patterns in Embedded Code

### Function prologue (entry)

```asm
sub sp, #12        ; reserve 12 bytes on the stack
str r0, [sp, #4]   ; save argument r0 to the stack
```

### Function epilogue (exit)

```asm
ldr r0, [sp, #0]   ; load return value into r0
add sp, #12        ; free the stack space
bx  lr             ; return to caller
```

### Volatile read (e.g. reading a hardware register)

```asm
ldr r0, [r0, #0]   ; read 32-bit value from the address in r0
```

> If `r0` holds an invalid or clock-disabled peripheral address, this causes a **HardFault**.

### Calling a function

```asm
bl  my_function    ; saves return address to lr, then jumps
bx  lr             ; inside my_function: return to caller
```

* * *

## Why These Appear in Embedded Rust

When Rust accesses peripheral registers via `read_volatile` / `write_volatile`,  
it compiles to these exact instructions:

| Rust | ARM Assembly |
| --- | --- |
| `read_volatile(ptr)` | `ldr r0, [r0, #0]` |
| `write_volatile(ptr, val)` | `str r1, [r0, #0]` |

This is why a bad pointer (wrong address, clock not enabled, unmapped memory)  
shows up as a HardFault at a `ldr` instruction inside `core::ptr::read_volatile`.

* * *

## Quick Mental Model

| You want to... | Instruction |
| --- | --- |
| Get data from memory | `ldr` |
| Put data into memory | `str` |
| Jump somewhere | `b` / `b.n` |
| Call a function | `bl` |
| Return from function | `bx lr` |
| Change a value | `add` / `sub` / `mov` |
| Compare two values | `cmp` (then use a `b.xx` branch) |
| Save registers before call | `push` |
| Restore registers after call | `pop` |

# Example

```as
Value returned has type: (cortex_m::peripheral::ITM, &stm32f3::stm32f303::gpioc::RegisterBlock). Cannot determine contents  
(gdb) disassemble /m  
Dump of assembler code for function registers::\__cortex_m_rt_main:  
21 fn main() -> ! {  
   0x08000210 &lt;+0&gt;: push {r7, lr}  
   0x08000212 &lt;+2&gt;: mov r7, sp

22 //let mut itm = aux7::init().0;  
23 let gpioe = aux7::init().1;  
   0x08000214 &lt;+4&gt;: bl 0x8000242 &lt;aux7::init&gt;  
\=> 0x08000218 &lt;+8&gt;: movw r0, #4120 @ 0x1018  
   0x0800021c &lt;+12&gt;: mov.w r1, #512 @ 0x200  
   0x08000220 &lt;+16&gt;: movt r0, #18432 @ 0x4800

24  
25 unsafe {  
26 // A magic address! // 0x48001000 (GPIOE base) + 0x18 (BSRR offset) = 0x48001018 // That's it ? simple addition of the peripheral base address and the register offset.  
27 const GPIOE_BSRR: u32 = 0x48001018;  
28  
29 // Print the initial content of ODR  
30 //iprint_odr(&mut itm);  
31  
32 // Turn on the "North" LED (red)  
33 ptr::write_volatile((GPIOE_BSRR as \*mut u32), 1 << 9);  
34 //iprint_odr(&mut itm);  
35 //gpioe.bsrr.write(|w| w.bs9().set_bit());  
36  
37 // Turn on the "East" LED (green)  
38 ptr::write_volatile((GPIOE_BSRR as \*mut u32), 1 << 11);  
39 //iprint_odr(&mut itm);  
40 //gpioe.bsrr.write(|w|w.bs11().set_bit());  
41  
\--Type &lt;RET&gt; for more, q to quit, c to continue without paging--  
42 // Turn off the "North" LED  
43 ptr::write_volatile((GPIOE_BSRR as \*mut u32), 1 << (9+16));  
44 //iprint_odr(&mut itm);  
45 //gpioe.bsrr.write(|w|w.bs9().set_bit());  
46  
47 // Turn off the "East" LED  
48 ptr::write_volatile((GPIOE_BSRR as \*mut u32), 1 << (11+16));  
49 //iprint_odr(&mut itm);  
50 //gpioe.bsrr.write(|w|w.bs11().set_bit());  
51 }  
52 /\* unsafe {  
53 // This address below is none exit so compiler will throw a Hard Fault.  
54 ptr::read_volatile(0x4800_1800 as \*const u32);  
55 }  
56 \*/  
57 loop {}  
   0x08000238 &lt;+40&gt;: b.n 0x8000238 &lt;registers::\__cortex_m_rt_main+40&gt;

End of assembler dump.
```

* * *

## **The Three-Step Read Method**

### **Step 1: Identify the address layout**

Every line follows this pattern:

```
   0x08000XXX <+OFFSET>:     INSTRUCTION     OPERANDS
```

- `0x08000XXX` = absolute memory address (flash ROM)
- `<+OFFSET>` = bytes from function start
- `INSTRUCTION` = what the CPU executes
- `OPERANDS` = what it operates on

**Your code:**

```
0x08000210 <+0>:     push    {r7, lr}        ? function starts here
0x08000212 <+2>:     mov     r7, sp          ? 2 bytes later
0x08000214 <+4>:     bl      0x8000242       ? 2 more bytes (Thumb is 2-byte aligned)
```

* * *

### **Step 2: Match assembly back to Rust source line numbers**

GDB shows line numbers on the left when you use `/m` flag:

```
21      fn main() -> ! {
   0x08000210 <+0>:     push    {r7, lr}    ? This is line 21 compiled

23          let gpioe = aux7::init().1;
   0x08000214 <+4>:     bl      0x8000242   ? This is line 23 compiled
```

**How to read it:**

1.  Find the Rust source line number on the left
2.  Read down until the next line number appears ? all assembly between is that Rust line

* * *

### **Step 3: Decode the instruction itself**

Learn 5 key instruction patterns. Here's **your exact code** decoded:

| Instruction | What it does | Your example |
| --- | --- | --- |
| `push {r7, lr}` | Save registers to stack | Save return address + frame pointer |
| `mov rD, rS` | Copy register S ? D | `mov r7, sp` copies stack pointer |
| `bl ADDRESS` | **B**ranch **L**ink ? jump to function, save return | `bl 0x8000242` calls `aux7::init()` |
| `movw rD, #IMMED` | Move **W**ide 16-bit immediate | `movw r0, #0x1018` = put lower 16 bits of address in r0 |
| `movt rD, #IMMED` | Move **T**op 16-bit immediate | `movt r0, #0x4800` = put upper 16 bits in r0 |
| `str rS, [rD]` | **ST**ore **R**egister ? write rS to memory at address in rD | Store your LED value to GPIO address |
| `b.n ADDRESS` | **B**ranch ? jump (no return) | `b.n 0x8000238` infinite loop |

* * *

## **Reading YOUR specific sequence**

```asm
23          let gpioe = aux7::init().1;
   0x08000214 <+4>:     bl      0x8000242 <aux7::init>
```

**Read as:** "Call the `aux7::init` function. Save return address in `lr` register."

```asm
   0x08000218 <+8>:     movw    r0, #4120       @ 0x1018
   0x0800021c <+12>:    mov.w   r1, #512        @ 0x200
   0x08000220 <+16>:    movt    r0, #18432      @ 0x4800
```

**Read as:**

- Load address `0x48001018` into `r0` (in two steps: lower half at `+8`, upper half at `+16`)
- Load value `0x200` (which is `1 << 9`) into `r1`

```asm
57          loop {}
   0x08000238 <+40>:    b.n     0x8000238 <registers::__cortex_m_rt_main+40>
```

**Read as:** "Jump back to address `0x8000238` (itself) ? infinite loop."

* * *

## **Quick-reference: Common ARM Thumb registers**

| Register | Purpose |
| --- | --- |
| `r0` | First arg / return value / temp |
| `r1` | Second arg / temp |
| `r7` | Frame pointer (optional, for debugging) |
| `sp` | Stack pointer (r13) |
| `lr` | Link register (r14) ? where to return to |
| `pc` | Program counter (r15) ? current instruction |

* * *

## **The Mental Model**

Think of disassembly as a **recipe with memory addresses**:

```
1. Save my return address (I might call a function)
2. Get the address 0x48001018 into register r0
3. Get the value 512 into register r1
4. Write r1 to the memory location in r0 (this writes to GPIO!)
5. Repeat forever
```

That's your whole program in plain English.

* * *

**Next: When you see the `str` instructions (after line 40), reply with them and I'll decode those too.**

&nbsp;

&nbsp;

&nbsp;