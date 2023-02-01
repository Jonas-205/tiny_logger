#![crate_name = "tiny_logger"]
//! This crate offers different logging macros,
//! that print out the messages in different colors
//! depending on how important they are

/// Used by print_color to decide how to print a message
pub enum LogLevel {
    Error,
    Warn,
    Info,
    /// Removed in release mode
    #[cfg(debug_assertions)]
    Debug,
    /// Removed in release mode
    #[cfg(debug_assertions)]
    Trace,
}

/// This is the "main" function used by all of the macros.
/// You should never have to call this directly.
#[inline(always)]
pub fn print_color(msg: &str, level: LogLevel) {
    use colored::Colorize;
    match level {
        LogLevel::Error => println!("{}", msg.on_red()),
        LogLevel::Warn => println!("{}", msg.yellow()),
        LogLevel::Info => println!("{}", msg.blue()),
        #[cfg(debug_assertions)]
        LogLevel::Debug => println!("{}", msg.on_yellow()),
        #[cfg(debug_assertions)]
        LogLevel::Trace => println!("{}", msg.normal()),
    }
}

/// Prints the message on red background and quits the program
#[macro_export]
macro_rules! die {
    ($($args: expr),*) => {
        $crate::print_color(&format!($($args),*), $crate::LogLevel::Error);
        panic!();
    }
}

/// Release: Nothing /
/// Debug: If the expression is false, prints the message on red background and quits the program
#[macro_export]
macro_rules! assert {
    ($base: expr) => {
        #[cfg(debug_assertions)]
        {
            if !$base {
                $crate::print_color("Assertion failed", $crate::LogLevel::Error);
                panic!();
            }
        }
    };
    ($base: expr, $($args: expr),*) => {
        #[cfg(debug_assertions)]
        {
            if !$base {
                $crate::print_color(&format!($($args),*), $crate::LogLevel::Error);
                panic!();
            }
        }
    }
}

/// Prints the message on red background
#[macro_export]
macro_rules! error {
    ($($args: expr),+) => {
        $crate::print_color(&format!($($args),*), $crate::LogLevel::Error);
    }
}

/// Prints the message in yellow
#[macro_export]
macro_rules! warn {
    ($($args: expr),+) => {
        $crate::print_color(&format!($($args),*), $crate::LogLevel::Warn);
    }
}

/// Prints a message in blue
#[macro_export]
macro_rules! info {
    ($($args: expr),+) => {
        $crate::print_color(&format!($($args),*), $crate::LogLevel::Info);
    }
}

/// (debug only) Prints a message on yellow and gives you the file / line number of the message:
/// [examples/logging.rs:23]: Debug
#[macro_export]
macro_rules! debug {
    ($($args: expr),+) => {
        #[cfg(debug_assertions)]
        {
            let msg = format!($($args),*);
            $crate::print_color(&format!("[{}:{}]: {}", file!(), line!(), msg), $crate::LogLevel::Debug);
        }
    };
}

/// (debug only) Normal println! in debug, nothing in release
#[macro_export]
macro_rules! trace {
    ($($args: expr),+) => {
        #[cfg(debug_assertions)]
        {
            $crate::print_color(&format!($($args),*), $crate::LogLevel::Trace);
        }
    }
}
