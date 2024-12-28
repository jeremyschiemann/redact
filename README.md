# Redactrs &emsp;  [![Latest Version]][crates.io] ![Total Downloads] [![MSRV]][Rust 1.60.0] ![License] [![Build Status]][actions] [![Documentation]][docs.rs]

[crates.io]: https://crates.io/crates/redactrs
[Latest Version]: https://img.shields.io/crates/v/redactrs 
[Total Downloads]: https://img.shields.io/crates/d/redactrs
[MSRV]: https://img.shields.io/crates/msrv/redactrs
[Rust 1.60.0]: https://blog.rust-lang.org/2022/04/07/Rust-1.60.0.html
[Build Status]: https://img.shields.io/github/actions/workflow/status/jeremyschiemann/redactrs/tests.yaml?branch=main
[actions]: https://github.com/jeremyschiemann/redactrs/actions?query=branch%3Amain
[Documentation]: https://img.shields.io/docsrs/redactrs
[docs.rs]: https://docs.rs/redactrs/
[License]: https://img.shields.io/crates/l/redactrs


## What is it?
Redactrs is a wrapper for sensitive data that you want to avoid being leaked by accidentally printing/logging/etc. them.

How the data is redacted is defined by the `Redactor`. A `Redactor` is a struct that implements the `Redactor`-trait.


## Usage

First add this crate to your project:
```
cargo add redactrs
```


In its most basic form, `Redacted` is used like this:
```rust
use redact::Redacted;

let x: Redacted<&str> = "sensitive".into();

assert_eq!(x.to_string(), "<redacted>");
```

This will by default use the `Simple`-Redactor. If desired, it can be swapped with the `Custom`-Redactor.

```rust
use redact::Redacted;
use redact::redactors::Custom;

let x: Redacted<&str, Custom<'X', 5>> = "sensitive".into();

assert_eq!(x.to_string(), "XXXXX");
```

To get back the wrapped type, you can either use `.into_inner()` which consumes the `Redacted` and returns the wrapped type
or use `.inner()`/`.inner_mut()` for a (non mutable) reference of the wrapped type.


## Serde support

Serde support can be activated by activating the `serde` feature!