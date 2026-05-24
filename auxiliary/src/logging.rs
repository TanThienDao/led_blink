#[allow(unused_imports)]
use cortex_m::iprintln;

/// Mutable context for tracking elapsed milliseconds during logging
/// This is a simple counter that increments with each log call.
pub struct LogContext {
    pub elapsed_ms: u32,
}

impl LogContext {
    /// Create a new logging context starting at 0ms
    pub fn new() -> Self {
        LogContext { elapsed_ms: 0 }
    }

    /// Increment the millisecond counter by 1
    /// Uses saturating_add to prevent overflow panics
    pub fn tick(&mut self) {
        self.elapsed_ms = self.elapsed_ms.saturating_add(1);
    }

    /// Reset counter to 0 (useful for testing or phase transitions)
    pub fn reset(&mut self) {
        self.elapsed_ms = 0;
    }
}

/// Log an info-level message with timestamp
///
/// Usage:
/// ```ignore
/// log_info!(itm, ctx, "Simple message");
/// log_info!(itm, ctx, "Message with var: {}", value);
/// ```
#[macro_export]
macro_rules! log_info {
    ($itm:expr, $ctx:expr, $msg:expr) => {
        {
            $ctx.tick();
            $crate::iprintln!(&mut $itm.stim[0], "[{:05}ms] INFO: {}", $ctx.elapsed_ms, $msg);
        }
    };
    ($itm:expr, $ctx:expr, $fmt:expr, $($arg:tt)*) => {
        {
            $ctx.tick();
            $crate::iprintln!(&mut $itm.stim[0], "[{:05}ms] INFO: {}", $ctx.elapsed_ms,
                format_args!($fmt, $($arg)*));
        }
    };
}

/// Log a warn-level message with timestamp
///
/// Usage:
/// ```ignore
/// log_warn!(itm, ctx, "Warning: LED not responding");
/// log_warn!(itm, ctx, "Timeout after {}ms", elapsed);
/// ```
#[macro_export]
macro_rules! log_warn {
    ($itm:expr, $ctx:expr, $msg:expr) => {
        {
            $ctx.tick();
            $crate::iprintln!(&mut $itm.stim[0], "[{:05}ms] WARN: {}", $ctx.elapsed_ms, $msg);
        }
    };
    ($itm:expr, $ctx:expr, $fmt:expr, $($arg:tt)*) => {
        {
            $ctx.tick();
            $crate::iprintln!(&mut $itm.stim[0], "[{:05}ms] WARN: {}", $ctx.elapsed_ms,
                format_args!($fmt, $($arg)*));
        }
    };
}

/// Log an error-level message with timestamp
///
/// Usage:
/// ```ignore
/// log_error!(itm, ctx, "Critical: System failure");
/// log_error!(itm, ctx, "Error code: {}", code);
/// ```
#[macro_export]
macro_rules! log_error {
    ($itm:expr, $ctx:expr, $msg:expr) => {
        {
            $ctx.tick();
            $crate::iprintln!(&mut $itm.stim[0], "[{:05}ms] ERROR: {}", $ctx.elapsed_ms, $msg);
        }
    };
    ($itm:expr, $ctx:expr, $fmt:expr, $($arg:tt)*) => {
        {
            $ctx.tick();
            $crate::iprintln!(&mut $itm.stim[0], "[{:05}ms] ERROR: {}", $ctx.elapsed_ms,
                format_args!($fmt, $($arg)*));
        }
    };
}

