// Menambah Validator Library
// `cargo add serde --features derive`
// `cargo add validator --features derive`

fn main() {
    println!("Hello, world!");
}

use validator::{Validate, ValidationErrors};
#[derive(Debug, Validate)]
struct User {
    #[validate(length(min = 3, message = "username terlalu pendek"))]
    username: String,

    #[validate(email(message = "email tidak valid"))]
    email: String,

    #[validate(range(min = 18, message = "usia minimal 18"))]
    age: u8,

    // #[validate()]
    // address: Address,
    #[validate(length(min = 1, message = "minimal 1 role"))]
    roles: Vec<String>,
}
// #[derive(Debug, Validate)]
// struct Address {
//     #[validate(length(min = 5, message = "alamat terlalu pendek"))]
//     street: String,

//     #[validate(length(min = 3, message = "kota wajib diisi"))]
//     city: String,
// }

#[derive(Debug, Validate)]
struct LoginRequest {
    #[validate(length(
        min = 3,
        max = 20,
        message = "username must be between 3 and 20 characters"
    ))]
    username: String,
    #[validate(length(
        min = 3,
        max = 20,
        message = "password must be between 3 and 20 characters"
    ))]
    password: String,
}

#[test]
fn test_validate_success() {
    let login = LoginRequest {
        username: "eko".to_string(),
        password: "password".to_string(),
    };
    assert!(login.validate().is_ok())
}

#[test]
fn test_validate_failed() {
    let login = LoginRequest {
        username: "ek".to_string(),
        password: "p".to_string(),
    };
    // assert!(login.validate().is_ok());

    let errors: ValidationErrors = login.validate().err().unwrap();
    println!("isi errors: {:?}", errors)
}
