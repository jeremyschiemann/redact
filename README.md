# Redactrs

## What is it?
Redactrs is a wrapper for sensitive data that you want to avoid being leaked by accidentally printing/logging/etc. them.

How the data is redacted is defined by the `Redactor`. A `Redactor` is a struct that implements the `Redactor`-trait.

## Usage

In it's most basic form, `Redacted` is used like this:
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

