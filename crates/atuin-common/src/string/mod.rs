//! String-related utilities and extension traits.
#[cfg(feature = "unicode")]
pub mod ellipsis;
mod escape;

#[cfg(feature = "unicode")]
pub use ellipsis::EllipsizeExt;
pub use escape::EscapeNonPrintablePosixExt;
