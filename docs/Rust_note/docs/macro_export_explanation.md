# `#[macro_export]` in Rust

This note explains what `#[macro_export]` does in Rust, with examples based on `auxiliary/src/logging.rs`.

## Why `#[macro_export]` exists

By default, a `macro_rules!` macro is scoped to the module where it is defined.

`#[macro_export]` makes that macro available at the crate root so other modules/crates can use it.

In this project, macros like `log_info!`, `log_warn!`, and `log_error!` are defined in `auxiliary/src/logging.rs`, but because of `#[macro_export]`, they can be used as crate-level macros.

## Without vs with `#[macro_export]`

### Without

```rust
mod logging {
    macro_rules! log_info {
        ($msg:expr) => {{ /* ... */ }};
    }
}

// In another module/crate: not visible
// log_info!("hello"); // error
```

### With

```rust
mod logging {
    #[macro_export]
    macro_rules! log_info {
        ($msg:expr) => {{ /* ... */ }};
    }
}

// Now macro is exported at crate root
// log_info!("hello");
```

## Why `$crate::...` is used inside exported macros

In `logging.rs`, the macros call:

```rust
$crate::iprintln!(...)
```

`$crate` always refers to the crate where the macro is defined, even when called from another crate/module.

This avoids name-resolution issues and is the recommended pattern for exported macros.

---
## Macro arms in this project

Each logging macro has two arms:

- Simple message arm (e.g. `log_info!(itm, ctx, "boot")`)
- Formatted arm (e.g. `log_info!(itm, ctx, "value: {}", x)`)

Example pattern:

```rust
($itm:expr, $ctx:expr, $msg:expr) => { ... };
($itm:expr, $ctx:expr, $fmt:expr, $($arg:tt)*) => { ... };
```
### The two arms pattern in your macros
Each macro in logging.rs has two arms:
```
macro_rules! log_info {
    // Arm 1: plain message
    ($itm:expr, $ctx:expr, $msg:expr) => { ... };

    // Arm 2: formatted message
    ($itm:expr, $ctx:expr, $fmt:expr, $($arg:tt)*) => { ... };
}
```
Rust matches arms top to bottom, first match wins.

| You write | Arm matched |
| :--- | :--- |
| `log_info!(itm, ctx, "hello")` | Arm 1 (3 tokens) |
| `log_info!(itm, ctx, "val={}", x)` | Arm 2 (3+ tokens) |
| `log_info!(itm, ctx)` | Arm 3 (2 tokens) |


Why not just use one arm?

You could, but you'd have to do something awkward like always pass an extra argument. Having two arms mirrors how println! itself works and is idiomatic Rust.

---
### Meta-Variable Types (`:expr`, `:tt`, etc.)

Inside a macro arm, each captured piece has a fragment specifier after the colon:


| Specifier | What it accepts |
| :--- | :--- |
| `:expr` | Any expression: `x`, `1+2`, `foo()` |
| `:ident` | An identifier: `my_var`, `SomeType` |
| `:tt` | A single token tree (the most flexible) |
| `:ty` | A type: `u32`, `Vec<i32>` |
| `:literal` | A literal value: `"hello"`, `42` |
| `:stmt` | A statement |
| `:block` | A `{ ... }` block |

### In Your Macros:

* `$itm:expr` — Captures the ITM object expression
* `$ctx:expr` — Captures the log context expression
* `$fmt:expr` — Captures the format string
* `$($arg:tt)*` — Captures zero or more token trees (the variadic part)

The `$()*` syntax means **"repeat this pattern zero or more times"**. That is how `println!` style variadic arguments work in Rust macros.

---
## `format_args!` in formatted logging arms

The formatted arms use:

```rust
format_args!($fmt, $($arg)*)
```

`format_args!` builds formatting arguments without heap-allocating a `String`, which is ideal for embedded/no-std style logging.
```
// BAD for embedded — allocates a String on the heap
let s = format!("val={}", x);
iprintln!(..., "{}", s);

// GOOD for embedded — zero allocation
iprintln!(..., "{}", format_args!("val={}", x));
```

---
## Small note about current `log_warn!`

In `auxiliary/src/logging.rs`, the formatted `log_warn!` arm currently prints twice:

1. Once using `$crate::iprintln!`
2. Once using plain `iprintln!`

That second line is likely accidental and may cause duplicate warning output.

---

If needed, this file can be expanded with a section on macro hygiene and token matchers (`:expr`, `:tt`, repetition with `$(...)*`).

