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

use std::fmt;

//
// Extension trait for Result types.
//

pub trait ResultExt<T, E> {
    /// Unwraps a result, yielding the content of an [`Ok`].
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`Err`], logging a message provided by the
    /// [`Err`]'s value to a [`slog::Logger`] at a [`Critical`] level.
    ///
    /// [`Critical`]: /slog/2/slog/2/slog/enum.Level.html#variant.Critical
    fn unwrap_or_log(self, log: &slog::Logger) -> T
    where
        E: fmt::Debug;

    /// Unwraps a result, yielding the content of an [`Ok`].
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`Err`], logging the passed message and the
    /// content of the [`Err`] to a [`slog::Logger`] at a [`Critical`] level.
    ///
    /// [`Critical`]: /slog/2/slog/enum.Level.html#variant.Critical
    fn expect_or_log(self, log: &slog::Logger, msg: &str) -> T
    where
        E: fmt::Debug;

    /// Unwraps a result, yielding the content of an [`Err`].
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`Ok`], logging a message provided by the
    /// [`Ok`]'s value to a [`slog::Logger`] at a [`Critical`] level.
    ///
    /// [`Critical`]: /slog/2/slog/enum.Level.html#variant.Critical
    fn unwrap_err_or_log(self, log: &slog::Logger) -> E
    where
        T: fmt::Debug;

    /// Unwraps a result, yielding the content of an [`Err`].
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`Ok`], logging the passed message and the
    /// content of the [`Ok`] to a [`slog::Logger`] at a [`Critical`] level.
    ///
    /// [`Critical`]: /slog/2/slog/enum.Level.html#variant.Critical
    fn expect_err_or_log(self, log: &slog::Logger, msg: &str) -> E
    where
        T: fmt::Debug;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    #[inline]
    // #[track_caller]
    fn unwrap_or_log(self, log: &slog::Logger) -> T
    where
        E: fmt::Debug,
    {
        match self {
            Ok(t) => t,
            Err(e) => failed_with(
                log,
                "called `Result::unwrap_or_log()` on an `Err` value",
                &e,
            ),
        }
    }

    #[inline]
    // #[track_caller]
    fn expect_or_log(self, log: &slog::Logger, msg: &str) -> T
    where
        E: fmt::Debug,
    {
        match self {
            Ok(t) => t,
            Err(e) => failed_with(log, msg, &e),
        }
    }

    #[inline]
    // #[track_caller]
    fn unwrap_err_or_log(self, log: &slog::Logger) -> E
    where
        T: fmt::Debug,
    {
        match self {
            Ok(t) => failed_with(
                log,
                "called `Result::unwrap_err_or_log()` on an `Ok` value",
                &t,
            ),
            Err(e) => e,
        }
    }

    #[inline]
    // #[track_caller]
    fn expect_err_or_log(self, log: &slog::Logger, msg: &str) -> E
    where
        T: fmt::Debug,
    {
        match self {
            Ok(t) => failed_with(log, msg, &t),
            Err(e) => e,
        }
    }
}

//
// Helper functions.
//

#[inline(never)]
#[cold]
// #[track_caller]
fn failed(log: &slog::Logger, msg: &str) -> ! {
    slog::crit!(log, "{}", msg);

    #[cfg(feature = "panic-quiet")]
    panic!();
    #[cfg(not(feature = "panic-quiet"))]
    panic!("{}", msg)
}

#[inline(never)]
#[cold]
// #[track_caller]
fn failed_with(log: &slog::Logger, msg: &str, value: &dyn fmt::Debug) -> ! {
    slog::crit!(log, "{}: {:?}", msg, &value);

    #[cfg(feature = "panic-quiet")]
    panic!();
    #[cfg(not(feature = "panic-quiet"))]
    panic!("{}: {:?}", msg, &value);
}
