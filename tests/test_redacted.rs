use redactrs::redactors::{Custom, Simple};
use redactrs::Redacted;

#[test]
fn display() {
    let x: Redacted<_, Simple> = 42.into();
    assert_eq!(format!("{}", x), "<redacted>");
}

#[test]
fn debug() {
    let x: Redacted<_, Simple> = 42.into();
    assert_eq!(format!("{:?}", x), "<redacted>");
}

#[test]
fn simple() {
    let x: Redacted<_, Simple> = 42.into();
    assert_eq!(x.to_string(), "<redacted>");
}

#[test]
fn custom() {
    let x: Redacted<_, Custom<'☠', 3>> = 42.into();
    assert_eq!(x.to_string(), "☠☠☠")
}

#[test]
fn custom_default() {
    let x: Redacted<_, Custom> = 42.into();
    assert_eq!(x.to_string(), "●●●●●●●●")
}

#[test]
fn clone() {
    let simple: Redacted<_, Simple> = 42.into();
    let _: Redacted<_, Simple> = simple.clone();
}

#[test]
fn copy() {
    let simple: Redacted<_, Simple> = 42.into();
    let simple_copy: Redacted<_, Simple> = simple;

    assert_eq!(simple.to_string(), simple_copy.to_string());
}

#[test]
fn size() {
    assert_eq!(size_of::<i32>(), size_of::<Redacted<i32>>());
}

#[test]
fn default() {
    let simple: Redacted<i32, Simple> = Default::default();
    assert_eq!(simple.to_string(), "<redacted>");
    assert_eq!(simple.into_inner(), 0);
}

#[test]
fn equals_with_redacted() {
    let x: Redacted<i32, Simple> = 42.into();
    let y: Redacted<i32, Simple> = 42.into();
    assert_eq!(x, y);
}

#[test]
fn equals_with_inner() {
    let x: Redacted<i32, Simple> = 42.into();
    let y: i32 = 42;
    assert_eq!(x, y);
}

#[test]
fn order_with_redacted() {
    let x: Redacted<i32, Simple> = 42.into();
    let y: Redacted<i32, Simple> = 24.into();
    assert!(x > y);
    assert!(x >= y);
    assert!(!(x < y));
    assert!(!(x <= y));
}

#[test]
fn order_with_inner() {
    let x: Redacted<i32, Simple> = 42.into();
    let y: i32 = 24;
    assert!(x > y);
    assert!(x >= y);
    assert!(!(x < y));
    assert!(!(x <= y));
}
