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
    fn unwrap_log(self, log: &slog::Logger) -> T
    where
        E: fmt::Debug;

    /// Unwraps a result, yielding the content of an [`Ok`].
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`Err`], logging the passed message and the
    /// content of the [`Err`] to a [`slog::Logger`] at a [`Critical`] level.
    fn expect_log(self, log: &slog::Logger, msg: &str) -> T
    where
        E: fmt::Debug;

    /// Unwraps a result, yielding the content of an [`Err`].
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`Ok`], logging a message provided by the
    /// [`Ok`]'s value to a [`slog::Logger`] at a [`Critical`] level.
    fn unwrap_err_log(self, log: &slog::Logger) -> E
    where
        T: fmt::Debug;

    /// Unwraps a result, yielding the content of an [`Err`].
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`Ok`], logging the passed message and the
    /// content of the [`Ok`] to a [`slog::Logger`] at a [`Critical`] level.
    fn expect_err_log(self, log: &slog::Logger, msg: &str) -> E
    where
        T: fmt::Debug;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn unwrap_log(self, log: &slog::Logger) -> T
    where
        E: fmt::Debug,
    {
        match self {
            Ok(t) => t,
            Err(e) => unwrap_failed(log, "called `Result::unwrap_log()` on an `Err` value", &e),
        }
    }

    fn expect_log(self, log: &slog::Logger, msg: &str) -> T
    where
        E: fmt::Debug,
    {
        match self {
            Ok(t) => t,
            Err(e) => unwrap_failed(log, msg, &e),
        }
    }

    fn unwrap_err_log(self, log: &slog::Logger) -> E
    where
        T: fmt::Debug,
    {
        match self {
            Ok(t) => unwrap_failed(
                log,
                "called `Result::unwrap_err_log()` on an `Ok` value",
                &t,
            ),
            Err(e) => e,
        }
    }

    fn expect_err_log(self, log: &slog::Logger, msg: &str) -> E
    where
        T: fmt::Debug,
    {
        match self {
            Ok(t) => unwrap_failed(log, msg, &t),
            Err(e) => e,
        }
    }
}

fn unwrap_failed(log: &slog::Logger, msg: &str, error: &dyn fmt::Debug) -> ! {
    slog::crit!(log, "{}: {:?}", msg, &error);
    panic!();
}
