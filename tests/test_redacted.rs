use redactrs::redactors::{Custom, Simple};
use redactrs::Redacted;
use serde::Serialize;
use zeroize::Zeroize;

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

#[test]
fn serialize() {
    use serde::Serialize;
    #[derive(Serialize)]
    struct MyData {
        a: Redacted<i32>,
        b: i32,
    }

    let data = MyData {
        a: 42.into(),
        b: 24,
    };

    let json = serde_json::to_string(&data).expect("Test case");

    assert_eq!(json, r#"{"a":"<redacted>","b":24}"#);
}

#[test]
fn serialize_no_redact() {
    use redactrs::serde::no_redact;
    use serde::Serialize;
    #[derive(Serialize)]
    struct MyData {
        #[serde(serialize_with = "no_redact")]
        a: Redacted<i32>,
        b: i32,
    }

    let data = MyData {
        a: 42.into(),
        b: 24,
    };

    let json = serde_json::to_string(&data).expect("Test case");

    assert_eq!(json, r#"{"a":42,"b":24}"#);
}

#[test]
fn deserialize() {
    use serde::Deserialize;
    #[derive(Deserialize)]
    struct MyData {
        a: Redacted<i32>,
        b: i32,
    }

    let data: MyData = serde_json::from_str(r#"{"a":42,"b":24}"#).expect("Test case");

    assert_eq!(data.a, 42);
    assert_eq!(data.b, 24);
    assert_eq!(data.a.to_string(), "<redacted>");
}

#[test]
fn zeroize() {
    let mut x: Redacted<i32> = 42.into();
    x.zeroize();

    assert_eq!(*x.inner(), 0);
}
