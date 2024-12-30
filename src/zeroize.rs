//! Module with zeroize Trait impls.
//!
//! [Zeroize](https://docs.rs/zeroize/latest/zeroize/trait.Zeroize.html),
//! [TryZeroize](https://docs.rs/zeroize/latest/zeroize/trait.TryZeroize.html)
//! and [ZeroizeOnDrop](https://docs.rs/zeroize/latest/zeroize/trait.ZeroizeOnDrop.html) are auto implemented for all `Redacted<T>`
//! where `T` implements the trait.

use crate::{Redacted, Redactor};
use zeroize::{TryZeroize, Zeroize, ZeroizeOnDrop};

#[cfg(any(feature = "zeroize", doc))]
impl<T, R> Zeroize for Redacted<T, R>
where
    T: Zeroize,
    R: Redactor,
{
    fn zeroize(&mut self) {
        self.inner.zeroize()
    }
}

#[cfg(any(feature = "zeroize", doc))]
impl<T, R> TryZeroize for Redacted<T, R>
where
    T: TryZeroize,
    R: Redactor,
{
    fn try_zeroize(&mut self) -> bool {
        self.inner.try_zeroize()
    }
}

#[cfg(any(feature = "zeroize", doc))]
impl<T, R> ZeroizeOnDrop for Redacted<T, R>
where
    T: ZeroizeOnDrop,
    R: Redactor,
{
}
