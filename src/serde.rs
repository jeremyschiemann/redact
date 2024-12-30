//! Module with serde related functionality.
//!
//! Requires feature `serde`.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::{Redacted, Redactor};


/// Requires feature `serde`.
impl<T, R> Serialize for Redacted<T, R>
where
    T: Serialize,
    R: Redactor,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}


/// Requires feature `serde`.
impl<'de, T, R> Deserialize<'de> for Redacted<T, R>
where
    T: Deserialize<'de>,
    R: Redactor,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: T = Deserialize::deserialize(deserializer)?;
        Ok(value.into())
    }
}


/// Use this function if you want serde to serialize without redaction.
///
/// ```rust
/// use serde::Serialize;
/// use redactrs::serde::no_redact;
/// use redactrs::Redacted;
///
/// #[derive(Serialize)]
/// struct MyData {
///     #[serde(serialize_with = "no_redact" )]
///     a: Redacted<i32>,
/// }
/// ```
///
/// Requires feature `serde`.
pub fn no_redact<T, R, S>(value: &Redacted<T, R>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
    R: Redactor,
{
    value.inner.serialize(serializer)
}
