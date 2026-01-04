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

// ## Nested struct
#[derive(Debug, Validate)]
struct AddressRequest {
    #[validate(length(
        min = 1,
        max = 100,
        message = "\"street\" length must be between 1 to 100"
    ))]
    street: String,
    #[validate(length(
        min = 1,
        max = 100,
        message = "\"city\" length must be between 1 to 100"
    ))]
    city: String,
    #[validate(length(
        min = 1,
        max = 100,
        message = "\"country\" length must be between 1 to 100"
    ))]
    country: String,
}
#[derive(Debug, Validate)]
struct RegisterUserRequest {
    #[validate(length(
        min = 3,
        max = 20,
        message = "\"username\" length must be between 3 to 20"
    ))]
    username: String,
    #[validate(length(
        min = 8,
        max = 20,
        message = "\"password\" length must be between 8 to 20"
    ))]
    password: String,
    #[validate(length(
        min = 3,
        max = 100,
        message = "\"name\" length must be between 3 to 100"
    ))]
    name: String,
    // validasi nested disini
    #[validate(nested)]
    address: AddressRequest,
}

#[test]
fn test_register_user_validate_success() {
    let addr = AddressRequest {
        street: "Jl. Merdeka 1".to_string(),
        city: "Jakarta".to_string(),
        country: "Indonesia".to_string(),
    };
    let req = RegisterUserRequest {
        username: "eko".to_string(),
        password: "secret123".to_string(),
        name: "Eko".to_string(),
        address: addr,
    };
    assert!(req.validate().is_ok());
}

#[test]
fn test_register_user_validate_failed_address() {
    let addr = AddressRequest {
        street: "".to_string(),
        city: "".to_string(),
        country: "".to_string(),
    };
    let req = RegisterUserRequest {
        username: "eko".to_string(),
        password: "secret123".to_string(),
        name: "Eko".to_string(),
        address: addr,
    };
    let errors = req.validate().err().unwrap();
    println!("error: {:?}", errors);
    let debug = format!("{:?}", errors);
    assert!(debug.contains("street"));
    assert!(debug.contains("city"));
    assert!(debug.contains("country"));
}

#[test]
fn test_register_user_validate_multiple_failures() {
    let addr = AddressRequest {
        street: "S".to_string(), // too short
        city: "Y".to_string(),   // too short
        country: "".to_string(), // empty
    };
    let req = RegisterUserRequest {
        username: "ab".to_string(),   // too short
        password: "pass".to_string(), // too short
        name: "Al".to_string(),       // too short
        address: addr,
    };
    let errors = req.validate().err().unwrap();
    println!("error: {:?}", errors);
    let debug = format!("{:?}", errors);
    assert!(debug.contains("username"));
    assert!(debug.contains("password"));
    assert!(debug.contains("name"));
    // nested address errors should also be present
    assert!(debug.contains("street") || debug.contains("city") || debug.contains("country"));
}
