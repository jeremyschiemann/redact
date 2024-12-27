#![deny(missing_docs)]
//! Wrapper for sensitive data that you want to avoid being leaked by accidentally printing/logging/etc. them.
//!
//! How the data is redacted is defined by the [Redactor]. A Redactor is a struct that implements the [Redactor]-trait.
//! # Usage
//!
//! In it's most basic form, [Redacted] is used like this:
//! ```rust
//! # use redactrs::Redacted;
//! let x: Redacted<&str> = "sensitive".into();
//!
//! assert_eq!(x.to_string(), "<redacted>");
//! ```
//!
//! This will by default use the [Simple]-Redactor. If desired, it can be swapped with the [Custom]-Redactor.
//!
//! ```rust
//! # use redactrs::Redacted;
//! # use redactrs::redactors::Custom;
//! let x: Redacted<&str, Custom<'X', 5>> = "sensitive".into();
//!
//! assert_eq!(x.to_string(), "XXXXX");
//! ```
//!
//! To get back the wrapped type use:
//! - `.into_inner()` which consumes the [Redacted] and returns the wrapped type.
//! - `.inner()` to get a reference of the wrapped type.
//! - `.inner_mut()` to get a mutable reference of the wrapped tyepe.
//!
//! NOTE: a [Redacted] is always constructed via the [From]/[Into] trait.
pub mod redactors;

use crate::redactors::Simple;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter, Result};
use std::marker::PhantomData;

#[cfg(doc)]
use crate::redactors::*;

/// A Trait to define how a value should be redacted.
pub trait Redactor {
    ///Function called by [Display] and [Debug].
    fn redact(f: &mut Formatter<'_>) -> Result
    where
        Self: Sized;
}

/// Struct used to wrap sensitive content that should not be printed/logged.
/// The redaction behaviour is defined by [Redactor].
///
/// ```rust
/// # use redactrs::Redacted;
/// let secret: Redacted<String> = "my_secret_string".to_string().into();
///
/// assert_eq!(secret.to_string(), "<redacted>");
/// ```
///
/// ```rust
/// # use redactrs::Redacted;
/// # use redactrs::redactors::Custom;
/// let secret: Redacted<String, Custom> = "my_secret_string".to_string().into();
///
/// assert_eq!(secret.to_string(), "●●●●●●●●");
/// ```
pub struct Redacted<T, R: Redactor = Simple> {
    inner: T,
    _redactor: PhantomData<R>,
}

impl<T, R: Redactor> Redacted<T, R> {
    ///Consumes the [Redacted], returning the wrapped value.
    ///```rust
    /// # use redactrs::Redacted;
    /// let x: Redacted<_> = "something".into();
    /// assert_eq!(x.into_inner(), "something");
    /// ```
    pub fn into_inner(self) -> T {
        self.inner
    }

    ///Get a reference to the wrapped type.
    ///```rust
    /// # use redactrs::Redacted;
    /// let x: Redacted<_> = "something".into();
    /// assert_eq!(*x.inner(), "something");
    ///```
    pub fn inner(&self) -> &T {
        &self.inner
    }

    ///Get a mutable reference to the wrapped type.
    ///```rust
    /// # use redactrs::Redacted;
    /// let mut x: Redacted<_> = "something".into();
    /// *x.inner_mut() = "different";
    ///
    /// assert_eq!(*x.inner(), "different");
    ///```
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T: Default, R: Redactor> Default for Redacted<T, R> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
            _redactor: PhantomData,
        }
    }
}

impl<T, R: Redactor> From<T> for Redacted<T, R> {
    fn from(value: T) -> Self {
        Redacted {
            inner: value,
            _redactor: PhantomData,
        }
    }
}

impl<T: Clone, R: Redactor> Clone for Redacted<T, R> {
    fn clone(&self) -> Self {
        Redacted {
            inner: self.inner.clone(),
            _redactor: PhantomData,
        }
    }
}

impl<T: Copy, R: Redactor> Copy for Redacted<T, R> {}

impl<T: PartialEq, R: Redactor> PartialEq for Redacted<T, R> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<T: PartialEq, R: Redactor> PartialEq<T> for Redacted<T, R> {
    fn eq(&self, other: &T) -> bool {
        self.inner.eq(other)
    }
}

impl<T: Eq, R: Redactor> Eq for Redacted<T, R> {}

impl<T: PartialOrd, R: Redactor> PartialOrd for Redacted<T, R> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl<T: PartialOrd, R: Redactor> PartialOrd<T> for Redacted<T, R> {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.inner().partial_cmp(other)
    }
}

impl<T, R: Redactor> Display for Redacted<T, R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        R::redact(f)
    }
}

impl<T, R: Redactor> Debug for Redacted<T, R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        R::redact(f)
    }
}
