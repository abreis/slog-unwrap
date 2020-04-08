## slog-unwrap
This crate provides `.unwrap_or_log()` and `.expect_or_log()` methods on `Result` and `Option` types that log failed unwraps to a [`slog::Logger`]. This is useful when, for example, you have a [syslog](https://github.com/slog-rs/syslog) drain or a database drain, and you want your unwrap failures to show up there instead of being printed to `stderr`.

Its API aims to mirror Rust's `std` — see all the [supported methods](#methods) below. Failed unwraps are logged at a level of [`Critical`].

[![crates.io](http://meritbadge.herokuapp.com/slog-unwrap)](https://crates.io/crates/slog-unwrap)
[![Documentation](https://docs.rs/slog-unwrap/badge.svg)](https://docs.rs/slog-unwrap)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/abreis/slog-unwrap)

### Usage
Add the following to your `Cargo.toml`:
```toml
slog-unwrap = "0.8"
```

Next, bring the [`ResultExt`] and/or [`OptionExt`] traits into scope, and make use of the new logging methods.
```rust
use slog-unwrap::ResultExt;

fn main() {
    let logger = ...;

    let not_great = Result::Err("not terrible");
    not_great.unwrap_or_log(&logger); // Logs the failed unwrap to `logger` and panics
}
```

### Methods
| `std` method                   | `slog-unwrap` form                      | trait         |
|--------------------------------| ----------------------------------------|---------------|
| [`Result::unwrap()`]           | [`Result::unwrap_or_log(&log)`]           | [`ResultExt`] |
| [`Result::expect(msg)`]        | [`Result::expect_or_log(&log, msg)`]      | [`ResultExt`] |
| [`Result::unwrap_err()`]       | [`Result::unwrap_err_or_log(&log)`]       | [`ResultExt`] |
| [`Result::expect_err(msg)`]    | [`Result::expect_err_or_log(&log, msg)`]  | [`ResultExt`] |
| [`Option::unwrap()`]           | [`Option::unwrap_or_log(&log)`]           | [`OptionExt`] |
| [`Option::expect(msg)`]        | [`Option::expect_or_log(&log, msg)`]      | [`OptionExt`] |
| [`Option::unwrap_none()`]<sup>†</sup>      | [`Option::unwrap_none_or_log(&log)`]      | [`OptionExt`] |
| [`Option::expect_none(msg)`]<sup>†</sup>   | [`Option::expect_none_or_log(&log, msg)`] | [`OptionExt`] |

*†: unstable in `std`*<br/>
*Note: enabling the `scope` feature drops the `&log` argument from all methods.*


### Features
* **`panic-quiet`**: causes failed unwraps to panic with an empty message.<br/>
  This feature is enabled by default — if you'd like the unwrap error message to also show in the panic message, disable default features in your `Cargo.toml` as follows:<br/>
  `slog-unwrap = { version = "0.8", default-features = false }`
* **`scope`**: adds support for [`slog-scope`](https://github.com/slog-rs/scope), which removes the need to pass a [`slog::Logger`] to the various methods.


### Alternatives
See [slog-unwraps](https://crates.io/crates/slog_unwraps), another crate with a similar featureset.

[`slog::Logger`]: https://docs.rs/slog/*/slog/struct.Logger.html
[`ResultExt`]: https://docs.rs/slog-unwrap/*/slog_unwrap/trait.ResultExt.html
[`OptionExt`]: https://docs.rs/slog-unwrap/*/slog_unwrap/trait.OptionExt.html
[`Critical`]: https://docs.rs/slog/*/slog/enum.Level.html#variant.Critical
[`Result::unwrap()`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap
[`Result::expect(msg)`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.expect
[`Result::unwrap_err()`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_err
[`Result::expect_err(msg)`]: https://doc.rust-lang.org/std/result/enum.Result.html#method.expect_err
[`Option::unwrap()`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap
[`Option::expect(msg)`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.expect
[`Option::unwrap_none()`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_none
[`Option::expect_none(msg)`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.expect_none
[`Result::unwrap_or_log(&log)`]: https://docs.rs/slog-unwrap/*/slog_unwrap/trait.ResultExt.html#tymethod.unwrap_or_log
[`Result::expect_or_log(&log, msg)`]: https://docs.rs/slog-unwrap/*/slog_unwrap/trait.ResultExt.html#tymethod.expect_or_log
[`Result::unwrap_err_or_log(&log)`]: https://docs.rs/slog-unwrap/*/slog_unwrap/trait.ResultExt.html#tymethod.unwrap_err_or_log
[`Result::expect_err_or_log(&log, msg)`]: https://docs.rs/slog-unwrap/*/slog_unwrap/trait.ResultExt.html#tymethod.expect_err_or_log
[`Option::unwrap_or_log(&log)`]: https://docs.rs/slog-unwrap/*/slog_unwrap/trait.OptionExt.html#tymethod.unwrap_or_log
[`Option::expect_or_log(&log, msg)`]: https://docs.rs/slog-unwrap/*/slog_unwrap/trait.OptionExt.html#tymethod.expect_or_log
[`Option::unwrap_none_or_log(&log)`]: https://docs.rs/slog-unwrap/*/slog_unwrap/trait.OptionExt.html#tymethod.unwrap_none_or_log
[`Option::expect_none_or_log(&log, msg)`]: https://docs.rs/slog-unwrap/*/slog_unwrap/trait.OptionExt.html#tymethod.expect_none_or_log
