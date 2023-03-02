use ensure_assertions::{ensure_eq, ensure_ne};
use anyhow::Result;

#[test]
fn test_one() {
    let result = test_one_assertions();
    assert!(result.is_err(), "returns error");
    assert_eq!(result.err().unwrap().to_string(), "assertion failed: `(left == right)`\n  left: `1`,\n right: `2`: 1 is equal to 2");
}

fn test_one_assertions() -> Result<()> {
    ensure_eq!(1, 2, "{}", "1 is equal to 2");
    ensure_eq!(1, 1, "{}", "1 is equal to 1");
    Ok(())
}

#[test]
fn test_two() {
    let result = test_two_assertions();
    assert!(result.is_ok(), "returns ok");
}

fn test_two_assertions() -> Result<()> {
    ensure_eq!(1, 1, "{}", "1 is equal to 1");
    ensure_eq!("abc", "abc", "{}", "abc is equal to abc");
    ensure_eq!(vec!["a", "b"], vec!["a", "b"], "{}", "vec! equal works");
    Ok(())
}

#[test]
fn test_three() {
    let result = test_three_assertions();
    assert!(result.is_err(), "returns error");
    assert_eq!(result.err().unwrap().to_string(), "assertion failed: `(left != right)`\n  left: `1`,\n right: `1`: 1 is not equal to 1");
}

fn test_three_assertions() -> Result<()> {
    ensure_ne!(1, 1, "{}", "1 is not equal to 1");
    ensure_ne!(1, 2, "{}", "1 is equal to 2");
    Ok(())
}

#[test]
fn test_four() {
    let result = test_four_assertions();
    assert!(result.is_ok(), "returns ok");
}

fn test_four_assertions() -> Result<()> {
    ensure_ne!(1, 2, "{}", "integer not equal works");
    ensure_ne!("abc", "def", "{}", "string not equal works");
    ensure_ne!(vec!["a", "b"], vec!["1", "2"], "{}", "vec! not equal works");
    Ok(())
}