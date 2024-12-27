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
//! # Feature flags
//! - `serde`: Enables serde support. Be aware: [Redacted] types will serialize into their redacted representation!
//!

pub mod redactors;

use crate::redactors::Simple;
#[cfg(doc)]
use crate::redactors::*;

#[cfg(any(feature = "serde", doc))]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter, Result};
use std::marker::PhantomData;

/// A Trait to define how a value should be redacted.
pub trait Redactor<T> {
    ///Function called by [Display] and [Debug].
    fn redact(value: &T, f: &mut Formatter<'_>) -> Result
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
pub struct Redacted<T, R: Redactor<T> = Simple> {
    inner: T,
    _redactor: PhantomData<R>,
}

impl<T, R: Redactor<T>> Redacted<T, R> {
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

impl<T: Default, R: Redactor<T>> Default for Redacted<T, R> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
            _redactor: PhantomData,
        }
    }
}

impl<T, R: Redactor<T>> From<T> for Redacted<T, R> {
    fn from(value: T) -> Self {
        Redacted {
            inner: value,
            _redactor: PhantomData,
        }
    }
}

impl<T: Clone, R: Redactor<T>> Clone for Redacted<T, R> {
    fn clone(&self) -> Self {
        Redacted {
            inner: self.inner.clone(),
            _redactor: PhantomData,
        }
    }
}

impl<T: Copy, R: Redactor<T>> Copy for Redacted<T, R> {}

impl<T: PartialEq, R: Redactor<T>> PartialEq for Redacted<T, R> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<T: PartialEq, R: Redactor<T>> PartialEq<T> for Redacted<T, R> {
    fn eq(&self, other: &T) -> bool {
        self.inner.eq(other)
    }
}

impl<T: Eq, R: Redactor<T>> Eq for Redacted<T, R> {}

impl<T: PartialOrd, R: Redactor<T>> PartialOrd for Redacted<T, R> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl<T: PartialOrd, R: Redactor<T>> PartialOrd<T> for Redacted<T, R> {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.inner().partial_cmp(other)
    }
}

impl<T: Ord, R: Redactor<T>> Ord for Redacted<T, R> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<T, R: Redactor<T>> Display for Redacted<T, R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        R::redact(&self.inner, f)
    }
}

impl<T, R: Redactor<T>> Debug for Redacted<T, R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        R::redact(&self.inner, f)
    }
}

/// Requires feature `serde`
#[cfg(any(feature = "serde", doc))]
impl<T: Serialize, R: Redactor<T>> Serialize for Redacted<T, R> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&*self.to_string())
    }
}

/// Requires feature `serde`
#[cfg(any(feature = "serde", doc))]
impl<'de, T: Deserialize<'de>, R: Redactor<T>> Deserialize<'de> for Redacted<T, R> {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: T = Deserialize::deserialize(deserializer)?;
        Ok(value.into())
    }
}
