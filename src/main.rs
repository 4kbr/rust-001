// install serde
// cargo add serde --features derive
// cargo add serde_json

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{
  Deserialize, Serialize, Serializer,
  de::{Error, Visitor},
};
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
  // serde bisa diconfig ganti nama attribut
  #[serde(rename = "alamat")]
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

// ubah array jadi string json
#[test]
fn test_create_json_from_array() {
  let numbers = [10, 11, 12, 13, 14];
  let json = serde_json::to_string(&numbers).unwrap();
  println!("{}", json);
}

#[derive(Debug, Serialize, Deserialize)] //`Serialize`,`Deserialize` wajib kalau mau pakai serde
#[serde(rename_all(
  serialize = "SCREAMING_SNAKE_CASE",
  deserialize = "SCREAMING_SNAKE_CASE"
))] // nanti jadi USERNAME, EMAIL, HOBBIES, dll.
struct User {
  username: String,
  first_name: Option<String>,
  email: String,
  hobbies: Vec<String>,
  phone: Option<String>, // ini bisa ada bisa tidak
  gender: Gender,
  payment: Payment,
}
#[derive(Serialize, Deserialize, Debug)]
enum Gender {
  Male,
  Female,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")] // diserder ini bisa diatur, 
// jadinya seperti ini
// ...,"PAYMENT":{"type":"BankAccount","account_number":"fRHE1Em0vogMf9Oh","bank_name":"Birdie Harris"}}
enum Payment {
  CreditCard {
    card_number: String,
    expiration: String,
  },
  BankAccount {
    account_number: String,
    bank_name: String,
  },
}

#[test]
fn test_vector() {
  let user: User = User {
    username: "testuser".to_string(),
    email: "test@gmail.com".to_string(),
    hobbies: vec!["reading".to_string(), "swimming".to_string()],
    phone: None,
    first_name: None,
    gender: Gender::Female,
    payment: Payment::BankAccount {
      account_number: "fRHE1Em0vogMf9Oh".to_string(),
      bank_name: "Birdie Harris".to_string(),
    },
  };

  let json: String = serde_json::to_string(&user).unwrap();
  println!("{}", json);

  let result: User = serde_json::from_str(&json).unwrap();
  println!("{:?}", result);
}

#[test]
fn test_vector_with_option() {
  let user: User = User {
    username: "testuser".to_string(),
    email: "test@gmail.com".to_string(),
    hobbies: vec!["reading".to_string(), "swimming".to_string()],
    phone: Some("+15525289".to_string()),
    first_name: None,
    gender: Gender::Male,
    payment: Payment::CreditCard {
      card_number: "Sxjr1xZDl".to_string(),
      expiration: "11/11/2095".to_string(),
    },
  };

  let json: String = serde_json::to_string(&user).unwrap();
  println!("{}", json);

  let result: User = serde_json::from_str(&json).unwrap();
  println!("{:?}", result);
}

// ## Map, untuk json yang key nya kita tidak ketahui

#[test]
fn test_map() {
  let mut values: HashMap<String, i32> = HashMap::new();
  values.insert("one".to_string(), 1);
  values.insert("two".to_string(), 2);
  values.insert("three".to_string(), 3);
  let json = serde_json::to_string(&values).unwrap();
  println!("json: {}", json);
  let result: HashMap<String, i32> = serde_json::from_str(&json).unwrap();
  println!("result: {:?}", result);
}

// ## Configurasi , serde bisa mengcustome attribute / nama parameter dari

// ## Chrono serde
// add crate chrono: `cargo add chrono --features serde`

#[derive(Serialize, Deserialize, Debug)]
struct Category {
  id: String,
  name: String,
  #[serde(with = "chrono::serde::ts_milliseconds")]
  created_at: DateTime<Utc>,
  #[serde(with = "chrono::serde::ts_milliseconds")]
  updated_at: DateTime<Utc>,
}

#[test]
fn test_chrono() {
  let category: Category = Category {
    id: "1DlitGey5ImOa8XKpc".to_string(),
    name: "Don Cobb".to_string(),
    created_at: Utc::now(),
    updated_at: Utc::now(),
  };

  let json: String = serde_json::to_string(&category).unwrap();
  println!("{}", json);

  let result: Category = serde_json::from_str(&json).unwrap();
  println!("{:?}", result);
}

// ## Custom serialization

#[derive(Debug, Serialize, Deserialize)]
struct Admin {
  id: String,
  // ini akan jadi name: "first last", tidak name: {first:,last:} karena sudah di custom
  name: Name,
}

#[derive(Debug)]
struct Name {
  first: String,
  last: String,
}

impl Serialize for Name {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(format!("{} {}", self.first, self.last).as_str())
  }
}

struct NameVisitor;
impl<'de> Visitor<'de> for NameVisitor {
  type Value = Name;
  fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
    formatter.write_str("Expecting name string")
  }

  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
  where
    E: Error,
  {
    let result: Vec<&str> = v.split(" ").collect();
    if result.len() != 2 {
      return Err(Error::custom("Expecting first and last name"));
    }
    Ok(Name {
      first: result[0].to_string(),
      last: result[1].to_string(),
    })
  }
}
impl<'de> Deserialize<'de> for Name {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    deserializer.deserialize_string(NameVisitor)
  }
}

#[test]
fn test_custom_serialization() {
  let admin = Admin {
    id: "123".to_string(),
    name: Name {
      first: "John".to_string(),
      last: "Doe".to_string(),
    },
  };
  let json = serde_json::to_string(&admin).unwrap();
  println!("json: {}", json);

  let result: Admin = serde_json::from_str(&json).unwrap();
  println!("result: {:?}", result);
}
