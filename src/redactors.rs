//! Contains ready to use [Redactor]s
use crate::Redactor;
use std::fmt::Formatter;

/// [Redactor] that will redact the value to "\<redacted\>"
/// ```rust
/// # use redactrs::Redacted;
/// # use redactrs::redactors::Simple;
/// let redacted_value: Redacted<_, Simple> = "secret".into();
///
/// assert_eq!(redacted_value.to_string(), "<redacted>")
/// ```
pub struct Simple;
impl Redactor for Simple {
    fn redact(f: &mut Formatter) -> std::fmt::Result {
        write!(f, "<redacted>")
    }
}

/// Configurable [Redactor] that lets you specify the symbol and repetition."
/// ```rust
/// # use redactrs::Redacted;
/// # use redactrs::redactors::Custom;
/// let redacted_value: Redacted<_, Custom<'🤨', 3>> = "secret".into();
///
/// assert_eq!(redacted_value.to_string(), "🤨🤨🤨");
/// ```
///
/// The default values will result in a password style redaction: "●●●●●●●●"
/// ```rust
/// # use redactrs::Redacted;
/// # use redactrs::redactors::Custom;
/// let redacted_value: Redacted<_, Custom> = "secret".into();
///
/// assert_eq!(redacted_value.to_string(),"●●●●●●●●");
/// ```
pub struct Custom<const SYMBOL: char = '●', const REP: usize = 8>;
impl<const SYMBOL: char, const REP: usize> Redactor for Custom<SYMBOL, REP> {
    fn redact(f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            std::iter::repeat(SYMBOL).take(REP).collect::<String>()
        )
    }
}
