# ARM/Thumb Disassembly Glossary

A beginner-friendly reference for reading ARM/Thumb assembly instructions,
as seen in GDB disassembly output when debugging embedded Rust on the STM32F3.

---

## How to read a disassembly line

```asm
0x0800174e <+2>:    str     r0, [sp, #4]
```

| Part | What it means |
|---|---|
| `0x0800174e` | Memory address where this instruction lives in flash |
| `<+2>` | Offset in bytes from the start of the current function |
| `str` | The instruction opcode (the operation to perform) |
| `r0` | Source register (operand) |
| `[sp, #4]` | Destination memory address = stack pointer + 4 bytes |

---

## Core Instructions

| Mnemonic | Full name | What it does | Example |
|---|---|---|---|
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

---

## Branch Instructions

Branches are "jump" instructions — they tell the CPU to go to a different address.

| Mnemonic | Full name | What it does | Example |
|---|---|---|---|
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

---

## Special Registers

| Register | Full name | What it holds |
|---|---|---|
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

---

## Memory Addressing Syntax

| Syntax | Meaning | Example |
|---|---|---|
| `[r0]` | Memory at address in `r0` | `ldr r1, [r0]` |
| `[r0, #4]` | Memory at address `r0 + 4` | `ldr r1, [r0, #4]` |
| `[sp, #8]` | Memory at address `sp + 8` | `str r0, [sp, #8]` |
| `[r0, #0]` | Memory at exact address in `r0` (offset 0) | `ldr r0, [r0, #0]` |
| `#12` | The number 12 directly (immediate value) | `sub sp, #12` |

> **Rule:** Brackets `[ ]` always mean "go to that memory address".  
> Without brackets, you're using the value directly.

---

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

---

## Why These Appear in Embedded Rust

When Rust accesses peripheral registers via `read_volatile` / `write_volatile`,
it compiles to these exact instructions:

| Rust | ARM Assembly |
|---|---|
| `read_volatile(ptr)` | `ldr r0, [r0, #0]` |
| `write_volatile(ptr, val)` | `str r1, [r0, #0]` |

This is why a bad pointer (wrong address, clock not enabled, unmapped memory)
shows up as a HardFault at a `ldr` instruction inside `core::ptr::read_volatile`.

---

## Quick Mental Model

| You want to... | Instruction |
|---|---|
| Get data from memory | `ldr` |
| Put data into memory | `str` |
| Jump somewhere | `b` / `b.n` |
| Call a function | `bl` |
| Return from function | `bx lr` |
| Change a value | `add` / `sub` / `mov` |
| Compare two values | `cmp` (then use a `b.xx` branch) |
| Save registers before call | `push` |
| Restore registers after call | `pop` |

