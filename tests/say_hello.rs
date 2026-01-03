#[cfg(test)]
// use my_crate::say_hello; // Ganti `my_crate` dengan nama crate Anda
use unit_test::*;

#[test]
fn test_hello_success() {
    let result = say_hello("Alice");
    assert_eq!(result, "Hello Alice");
}

#[test]
fn test_hello_empty_name() {
    let result = say_hello("");
    assert_eq!(result, "Hello ");
}

// #[test]
// fn test_hello_null_name() {
//     let result = say_hello(None);
//     assert_eq!(result, "Hello World");
// }

// #[test]
// #[should_panic(expected = "Name cannot be numeric")]
// fn test_hello_numeric_name() {
//     say_hello("123");
// }
