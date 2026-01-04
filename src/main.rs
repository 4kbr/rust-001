// install serde
// cargo add serde --features derive
// cargo add serde_json

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
struct UserLoginRequest {
  username: String,
  password: String,
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
