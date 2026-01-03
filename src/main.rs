fn main() {
    println!("Hello, world!");
}

// command untuk menjalankan test di terminal `cargo test nama_fn_test -- --show-output`

// `cargo test test_simple -- --show-output`
#[test]
fn test_simple() {
    println!("Hello test");
}

/* Assertion
`assert!(boolean, message)` // harus true
`assert_eq!(left,right,message)` // harus sama
`assert_ne!(left,right,message)` // harus beda

*/

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn test_add() {
    let result = add(2, 3);
    assert_eq!(result, 5, "2 + 3 should equal 5");
}

#[test]
fn test_add_negative() {
    let result = add(-2, -3);
    assert_eq!(result, -5, "-2 + -3 should equal -5");
}

#[test]
fn test_add_zero() {
    let result = add(0, 0);
    assert_eq!(result, 0, "0 + 0 should equal 0");
}

// contoh pakai assert_true
#[test]
fn test_assert_true() {
    let value = true;
    assert!(value, "Value should be true");
}

// contoh pakai assert_eq
#[test]
fn test_assert_eq() {
    let a = 5;
    let b = 5;
    assert_eq!(a, b, "a and b should be equal");
}

// contoh pakai assert_ne
#[test]
fn test_assert_ne() {
    let a = 5;
    let b = 10;
    assert_ne!(a, b, "a and b should not be equal");
}

// contoh pakai should_panic
#[test]
#[should_panic(expected = "This should panic")]
fn test_should_panic() {
    panic!("This should panic");
}

// contoh pakai ignored
#[test]
#[ignore]
fn test_ignored() {
    // This test will be ignored
    println!("This test is ignored");
}
