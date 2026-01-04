// install serde
// cargo add serde --features derive
// cargo add serde_json

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
struct UserLoginRequest {
  username: String,
  password: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AddressRequest {
  street: String,
  city: String,
  state: String,
  zip: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct CreateUserRequest {
  username: String,
  password: String,
  email: String,
  address: AddressRequest,
}

fn main() {
  println!("Hello, world!");
}

#[test]
fn test_create_json_for_user_login_request() {
  let login_request: UserLoginRequest = UserLoginRequest {
    username: "testuser".to_string(),
    password: "testpassword".to_string(),
  };

  let json = serde_json::to_string(&login_request).unwrap();
  println!("login_request json: {}", json);

  let login_result: UserLoginRequest = serde_json::from_str(&json).unwrap();
  println!("login_result: {:?}", login_result);
}

// test nested struct
#[test]
fn test_create_json_for_create_user_request() {
  let create_user_request: CreateUserRequest = CreateUserRequest {
    username: "testuser".to_string(),
    password: "testpassword".to_string(),
    email: String::from("kaeg@rop.py"),
    address: AddressRequest {
      street: "Barrett St.42".to_string(),
      city: "Kota".to_string(),
      state: "Portugal".to_string(),
      zip: "rbCtvx5aTHh".to_string(),
    },
  };

  let json = serde_json::to_string(&create_user_request).unwrap();
  println!("create_user_request json: {}", json);

  let create_user_result: CreateUserRequest = serde_json::from_str(&json).unwrap();
  println!("create_user_result: {:?}", create_user_result);
}
