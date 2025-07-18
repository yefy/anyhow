/// Return early with an error.
///
/// This macro is equivalent to `return Err(`[`anyhow!($args...)`][anyhow!]`)`.
///
/// The surrounding function's or closure's return value is required to be
/// `Result<_,`[`anyhow::Error`][crate::Error]`>`.
///
/// # Example
///
/// ```
/// # use anyhow::{bail, Result};
/// #
/// # fn has_permission(user: usize, resource: usize) -> bool {
/// #     true
/// # }
/// #
/// # fn main() -> Result<()> {
/// #     let user = 0;
/// #     let resource = 0;
/// #
/// if !has_permission(user, resource) {
///     bail!("permission denied for accessing {}", resource);
/// }
/// #     Ok(())
/// # }
/// ```
///
/// ```
/// # use anyhow::{bail, Result};
/// # use thiserror::Error;
/// #
/// # const MAX_DEPTH: usize = 1;
/// #
/// #[derive(Error, Debug)]
/// enum ScienceError {
///     #[error("recursion limit exceeded")]
///     RecursionLimitExceeded,
///     # #[error("...")]
///     # More = (stringify! {
///     ...
///     # }, 1).1,
/// }
///
/// # fn main() -> Result<()> {
/// #     let depth = 0;
/// #
/// if depth > MAX_DEPTH {
///     bail!(ScienceError::RecursionLimitExceeded);
/// }
/// #     Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! bail {
    ($msg:literal $(,)?) => {
        return $crate::private::Err($crate::anyhow!($msg))
    };
    ($err:expr $(,)?) => {
        return $crate::private::Err($crate::anyhow!($err))
    };
    ($fmt:expr, $($arg:tt)*) => {
        return $crate::private::Err($crate::anyhow!($fmt, $($arg)*))
    };
}

/// Return early with an error if a condition is not satisfied.
///
/// This macro is equivalent to `if !$cond { return
/// Err(`[`anyhow!($args...)`][anyhow!]`); }`.
///
/// The surrounding function's or closure's return value is required to be
/// `Result<_,`[`anyhow::Error`][crate::Error]`>`.
///
/// Analogously to `assert!`, `ensure!` takes a condition and exits the function
/// if the condition fails. Unlike `assert!`, `ensure!` returns an `Error`
/// rather than panicking.
///
/// # Example
///
/// ```
/// # use anyhow::{ensure, Result};
/// #
/// # fn main() -> Result<()> {
/// #     let user = 0;
/// #
/// ensure!(user == 0, "only user 0 is allowed");
/// #     Ok(())
/// # }
/// ```
///
/// ```
/// # use anyhow::{ensure, Result};
/// # use thiserror::Error;
/// #
/// # const MAX_DEPTH: usize = 1;
/// #
/// #[derive(Error, Debug)]
/// enum ScienceError {
///     #[error("recursion limit exceeded")]
///     RecursionLimitExceeded,
///     # #[error("...")]
///     # More = (stringify! {
///     ...
///     # }, 1).1,
/// }
///
/// # fn main() -> Result<()> {
/// #     let depth = 0;
/// #
/// ensure!(depth <= MAX_DEPTH, ScienceError::RecursionLimitExceeded);
/// #     Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! ensure {
    ($cond:expr $(,)?) => {
        if !$cond {
            return $crate::private::Err($crate::Error::msg(
                $crate::private::concat!("Condition failed: `", $crate::private::stringify!($cond), "`")
            ));
        }
    };
    ($cond:expr, $msg:literal $(,)?) => {
        if !$cond {
            return $crate::private::Err($crate::anyhow!($msg));
        }
    };
    ($cond:expr, $err:expr $(,)?) => {
        if !$cond {
            return $crate::private::Err($crate::anyhow!($err));
        }
    };
    ($cond:expr, $fmt:expr, $($arg:tt)*) => {
        if !$cond {
            return $crate::private::Err($crate::anyhow!($fmt, $($arg)*));
        }
    };
}

/// Construct an ad-hoc error from a string or existing non-`anyhow` error
/// value.
///
/// This evaluates to an [`Error`][crate::Error]. It can take either just a
/// string, or a format string with arguments. It also can take any custom type
/// which implements `Debug` and `Display`.
///
/// If called with a single argument whose type implements `std::error::Error`
/// (in addition to `Debug` and `Display`, which are always required), then that
/// Error impl's `source` is preserved as the `source` of the resulting
/// `anyhow::Error`.
///
/// # Example
///
/// ```
/// # type V = ();
/// #
/// use anyhow::{anyhow, Result};
///
/// fn lookup(key: &str) -> Result<V> {
///     if key.len() != 16 {
///         return Err(anyhow!("key length must be 16 characters, got {:?}", key));
///     }
///
///     // ...
///     # Ok(())
/// }
/// ```
#[macro_export]
macro_rules! anyhow {
    ($msg:literal $(,)?) => {
        // Handle $:literal as a special case to make cargo-expanded code more
        // concise in the common case.
        //$crate::Error::msg($msg)
        //$crate::Error::msg($crate::private::format!("@@@{}:{} {}", file!(), line!(), $msg))
        $crate::Error::msg($crate::private::format!("[{}:{} emsg({})]", file!(), line!(), $crate::private::format!($msg)))
    };
    ($err:expr $(,)?) => ({
        use $crate::private::kind::*;
        match $err {
            //error => (&error).anyhow_kind().new(error),
            error => $crate::Error::msg($crate::private::format!("[{}:{} emsg({})]", file!(), line!(), error))
        }
    });

    ($fmt:expr, $($arg:tt)*) => {
        //$crate::Error::msg($crate::private::format!($fmt, $($arg)*))
        //$crate::Error::msg($crate::private::format!($crate::private::concat!("@@@{}:{} ", $fmt), file!(), line!(), $($arg)*))
        $crate::Error::msg($crate::private::format!("[{}:{} emsg({})]", file!(), line!(), $crate::private::format!($fmt, $($arg)*)))
    };
    () => {
        $crate::Error::msg($crate::private::format!("[{}:{} emsg()]", file!(), line!()))
    };
}

#[macro_export]
macro_rules! anyhow_basic {
    ($msg:literal $(,)?) => {
        // Handle $:literal as a special case to make cargo-expanded code more
        // concise in the common case.
        $crate::Error::msg($msg)
    };
    ($err:expr $(,)?) => ({
        use $crate::private::kind::*;
        match $err {
            error => (&error).anyhow_kind().new(error),
        }
    });
    ($fmt:expr, $($arg:tt)*) => {
        $crate::Error::msg($crate::private::format!($fmt, $($arg)*))
    };
    () => {
        $crate::Error::msg($crate::private::format!(""))
    };
}

#[macro_export]
macro_rules! anyhow_error {
    // .map_err(anyhow_error!("some static msg"))
    ($msg:literal $(,)?) => {
        |e| ::anyhow::anyhow_basic!("[{}:{}, emsg({})]{}{:?}", file!(), line!(), $msg, anyhow::END_OF_LINE, e)
    };

    // .map_err(anyhow_error!("some format {}", value))
    ($fmt:expr, $($arg:tt)+) => {
        |e| ::anyhow::anyhow_basic!("[{}:{}, emsg({})]{}{:?}", file!(), line!(), format!($fmt, $($arg)+), anyhow::END_OF_LINE, e)
    };

    // anyhow_error!(err)
    ($err:expr $(,)?) => {
        |e| {
        let err = $err;
        ::anyhow::anyhow_basic!("[{}:{}, emsg({})]{}{:?}", file!(), line!(), err, anyhow::END_OF_LINE,e)
    }};
    // anyhow_error!()
    () => {
        |e| ::anyhow::anyhow_basic!("[{}:{}, emsg()]{}{:?}", file!(), line!(), anyhow::END_OF_LINE, e)
    };
}

#[macro_export]
macro_rules! anyhow_context {
    // .map_err(anyhow_context!("some static msg"))
    ($msg:literal $(,)?) => {
        |e| e.context(::anyhow::anyhow_basic!("[{}:{}, emsg({})]", file!(), line!(), $msg))
    };

    // .map_err(anyhow_context!("some format {}", value))
    ($fmt:expr, $($arg:tt)+) => {
        |e| e.context(::anyhow::anyhow_basic!("[{}:{}, emsg({})]", file!(), line!(), format!($fmt, $($arg)+)))
    };

    // anyhow_context!(err)
    ($err:expr $(,)?) => {
        |e| {
        let err = $err;
        e.context(::anyhow::anyhow_basic!("[{}:{}, emsg({})]", file!(), line!(), err))
    }};
    // anyhow_context!()
    () => {
        |e| e.context(::anyhow::anyhow_basic!("[{}:{}, emsg()]", file!(), line!()))
    };
}