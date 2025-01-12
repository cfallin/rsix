//! The `Error` type, which is a minimal wrapper around an error code.
//!
//! We define the error constants as individual `const`s instead of an
//! enum because we may not know about all of the host's error values
//! and we don't want unrecognized values to create UB.

use crate::imp;
use std::{error, fmt, result};

/// A specialized `Result` type for rsix APIs.
pub type Result<T> = result::Result<T, Error>;

/// `errno`—An error code.
///
/// The error type for rsix APIs. This is similar to `std::io::Error`, but
/// only holds an OS error code, and no extra error value.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/errno.html
/// [Linux]: https://man7.org/linux/man-pages/man3/errno.3.html
pub use imp::io::Error;

impl Error {
    /// Shorthand for `std::io::Error::from(self).kind()`.
    #[inline]
    pub fn kind(self) -> std::io::ErrorKind {
        std::io::Error::from(self).kind()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::io::Error::from(*self).fmt(fmt)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::io::Error::from(*self).fmt(fmt)
    }
}

impl error::Error for Error {}

impl From<Error> for std::io::Error {
    #[inline]
    fn from(err: Error) -> Self {
        Self::from_raw_os_error(err.raw_os_error() as _)
    }
}
