//! Extension traits for logging failed unwraps to a [`slog::Logger`].
//!
//! | `std` method               | `slog-unwrap` method                    | trait       |
//! |--------------------------- |---------------------------------------- |-------------|
//! | `Result::unwrap()`         | `Result::unwrap_or_log(&log)`           | `ResultExt` |
//! | `Result::expect(msg)`      | `Result::expect_or_log(&log, msg)`      | `ResultExt` |
//! | `Result::unwrap_err()`     | `Result::unwrap_err_or_log(&log)`       | `ResultExt` |
//! | `Result::expect_err(msg)`  | `Result::expect_err_or_log(&log, msg)`  | `ResultExt` |
//! | `Option::unwrap()`         | `Option::unwrap_or_log(&log)`           | `OptionExt` |
//! | `Option::expect(msg)`      | `Option::expect_or_log(&log, msg)`      | `OptionExt` |
//! | `Option::unwrap_none()`    | `Option::unwrap_none_or_log(&log)`      | `OptionExt` |
//! | `Option::expect_none(msg)` | `Option::expect_none_or_log(&log, msg)` | `OptionExt` |
//!
//! ## Features
//! `quiet-panic`


#[cfg(not(feature = "scope"))]
mod slog;
#[cfg(not(feature = "scope"))]
pub use crate::slog::*;

#[cfg(feature = "scope")]
mod scope;
#[cfg(feature = "scope")]
pub use crate::scope::*;
