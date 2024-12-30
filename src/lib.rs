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
//!
//!
//! # Serde support
//!
//! By default [Redacted] types will serialize into their redacted representation.
//! If you don't want this, and rather serialize normally
//! you can annotate the redacted field with this attribute: `#[serde(serialize_with = "no_redact")]`
//!
//! ```rust
//! use redactrs::Redacted;
//! use serde::Serialize;
//! use redactrs::serde::no_redact;
//! #[derive(Serialize)]
//! struct MyData {
//!     #[serde(serialize_with = "no_redact" )]
//!     a: Redacted<i32>,
//! }
//! let data = MyData {
//!     a: 42.into(),
//! };
//! let json = serde_json::to_string(&data).expect("Test case");
//! assert_eq!(json, r#"{"a":42}"#);
//! ```
//!
//! # Feature flags
//! - `serde`: Enables serde support.
//!

pub mod redactors;

#[cfg(any(feature = "serde", doc))]
pub mod serde;

use crate::redactors::Simple;
#[cfg(doc)]
use crate::redactors::*;

use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter, Result};
use std::marker::PhantomData;

/// A Trait to define how a value should be redacted.
pub trait Redactor {
    ///Function called by [Display] and [Debug].
    fn redact(f: &mut Formatter<'_>) -> Result;
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
pub struct Redacted<T, R = Simple>
where
    R: Redactor,
{
    inner: T,
    _redactor: PhantomData<R>,
}

impl<T, R> Redacted<T, R>
where
    R: Redactor,
{
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

impl<T, R> Default for Redacted<T, R>
where
    T: Default,
    R: Redactor,
{
    fn default() -> Self {
        Self {
            inner: Default::default(),
            _redactor: PhantomData,
        }
    }
}

impl<T, R> From<T> for Redacted<T, R>
where
    R: Redactor,
{
    fn from(value: T) -> Self {
        Redacted {
            inner: value,
            _redactor: PhantomData,
        }
    }
}

impl<T, R> Clone for Redacted<T, R>
where
    T: Clone,
    R: Redactor,
{
    fn clone(&self) -> Self {
        Redacted {
            inner: self.inner.clone(),
            _redactor: PhantomData,
        }
    }
}

impl<T, R> Copy for Redacted<T, R>
where
    T: Copy,
    R: Redactor,
{
}

impl<T, R> PartialEq for Redacted<T, R>
where
    T: PartialEq,
    R: Redactor,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<T, R> PartialEq<T> for Redacted<T, R>
where
    T: PartialEq,
    R: Redactor,
{
    fn eq(&self, other: &T) -> bool {
        self.inner.eq(other)
    }
}

impl<T, R> Eq for Redacted<T, R>
where
    T: Eq,
    R: Redactor,
{
}

impl<T, R> PartialOrd for Redacted<T, R>
where
    T: PartialOrd,
    R: Redactor,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl<T, R> PartialOrd<T> for Redacted<T, R>
where
    T: PartialOrd,
    R: Redactor,
{
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.inner().partial_cmp(other)
    }
}

impl<T, R> Ord for Redacted<T, R>
where
    T: Ord,
    R: Redactor,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<T, R> Display for Redacted<T, R>
where
    R: Redactor,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        R::redact(f)
    }
}

impl<T, R> Debug for Redacted<T, R>
where
    R: Redactor,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        R::redact(f)
    }
}
