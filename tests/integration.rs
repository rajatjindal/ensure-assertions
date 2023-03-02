use ensure_assertions::{ensure_eq, ensure_ne};
use anyhow::Result;

#[test]
fn test_one() {
    let result = test_one_assertions();
    assert!(result.is_err(), "returns error");
    assert_eq!(result.err().unwrap().to_string(), "assertion failed: `(left == right)`\n  left: `1`,\n right: `2`: this will trigger failure");
}

fn test_one_assertions() -> Result<()> {
    ensure_eq!(1, 2, "{}", "this will trigger failure");
    ensure_eq!(1, 1, "{}", "this will not be executed");
    Ok(())
}

#[test]
fn test_two() {
    let result = test_two_assertions();
    assert!(result.is_ok(), "returns ok");
}

fn test_two_assertions() -> Result<()> {
    ensure_eq!(1, 1, "{}", "this will pass");
    ensure_eq!("abc", "abc", "{}", "this will pass too");
    ensure_eq!(vec!["a", "b"], vec!["a", "b"], "{}", "this will pass too");
    Ok(())
}

#[test]
fn test_three() {
    let result = test_three_assertions();
    assert!(result.is_err(), "returns error");
    assert_eq!(result.err().unwrap().to_string(), "assertion failed: `(left != right)`\n  left: `1`,\n right: `1`: this will trigger failure");
}

fn test_three_assertions() -> Result<()> {
    ensure_ne!(1, 1, "{}", "this will trigger failure");
    ensure_ne!(1, 2, "{}", "this will not be executed");
    Ok(())
}

#[test]
fn test_four() {
    let result = test_four_assertions();
    assert!(result.is_ok(), "returns ok");
}

fn test_four_assertions() -> Result<()> {
    ensure_ne!(1, 2, "{}", "this will pass");
    ensure_ne!("abc", "def", "{}", "this will pass too");
    ensure_ne!(vec!["a", "b"], vec!["1", "2"], "{}", "this will pass too");
    Ok(())
}