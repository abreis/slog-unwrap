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
    /// [`Critical`]: /slog/2/slog/enum.Level.html#variant.Critical
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
// Extension trait for Option types.
//

pub trait OptionExt<T> {
    /// Moves the value `v` out of the `Option<T>` if it is [`Some(v)`].
    ///
    /// In general, because this function may panic, its use is discouraged.
    /// Instead, prefer to use pattern matching and handle the [`None`]
    /// case explicitly.
    ///
    /// # Panics
    ///
    /// Panics if the self value equals [`None`], logging an error message to a
    /// [`slog::Logger`] at a [`Critical`] level.
    fn unwrap_or_log(self, log: &slog::Logger) -> T;

    /// Unwraps an option, yielding the content of a [`Some`].
    ///
    /// # Panics
    ///
    /// Panics if the value is a [`None`], logging the passed message to a
    /// [`slog::Logger`] at a [`Critical`] level.
    fn expect_or_log(self, log: &slog::Logger, msg: &str) -> T;

    /// Unwraps an option, expecting [`None`] and returning nothing.
    ///
    /// # Panics
    ///
    /// Panics if the value is a [`Some`], logging a message derived from the
    /// [`Some`]'s value to a [`slog::Logger`] at a [`Critical`] level.
    fn unwrap_none_or_log(self, log: &slog::Logger)
    where
        T: fmt::Debug;

    /// Unwraps an option, expecting [`None`] and returning nothing.
    ///
    /// # Panics
    ///
    /// Panics if the value is a [`Some`], logging the passed message and the
    /// content of the [`Some`] to a [`slog::Logger`] at a [`Critical`] level.
    fn expect_none_or_log(self, log: &slog::Logger, msg: &str)
    where
        T: fmt::Debug;
}

impl<T> OptionExt<T> for Option<T> {
    fn unwrap_or_log(self, log: &slog::Logger) -> T {
        match self {
            Some(val) => val,
            None => failed(log, "called `Option::unwrap_or_log()` on a `None` value"),
        }
    }

    #[inline]
    // #[track_caller]
    fn expect_or_log(self, log: &slog::Logger, msg: &str) -> T {
        match self {
            Some(val) => val,
            None => failed(log, msg),
        }
    }

    #[inline]
    // #[track_caller]
    fn unwrap_none_or_log(self, log: &slog::Logger)
    where
        T: fmt::Debug,
    {
        if let Some(val) = self {
            failed_with(
                log,
                "called `Option::unwrap_none_or_log()` on a `Some` value",
                &val,
            );
        }
    }

    #[inline]
    // #[track_caller]
    fn expect_none_or_log(self, log: &slog::Logger, msg: &str)
    where
        T: fmt::Debug,
    {
        if let Some(val) = self {
            failed_with(log, msg, &val);
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
