use std::collections::HashMap;

use handlebars::Handlebars;
use serde_json::json;

// ## TEMPLATE
// pakai library `handlebars`
// `cargo add handlebars`
fn main() {
  println!("Hello, world!");
}

// ## Setup
// cargo test --package rust-template --bin rust-template -- test_handlebars --exact --nocapture
#[test]
fn test_handlebars() {
  let mut handlebars = handlebars::Handlebars::new();

  handlebars
    .register_template_string("hello", "Hello, {{name}}")
    .unwrap();

  handlebars
    .register_template_string("bye", "Bye, {{name}}")
    .unwrap();

  let mut data = std::collections::HashMap::new();
  data.insert("name", "Eko");
  let rendered = handlebars.render("hello", &data).unwrap();
  assert_eq!(rendered, "Hello, Eko");
  let rendered = handlebars.render("bye", &data).unwrap();
  assert_eq!(rendered, "Bye, Eko");
}

// ## Nested Variable

// cargo test --package rust-template --bin rust-template -- test_nested_variable --exact --nocapture
#[test]
fn test_nested_variable() {
  let mut handlebars = Handlebars::new();
  handlebars
    .register_template_string("hello", "Hello, {{person.first_name}} {{person.last_name}}")
    .unwrap();

  let mut data = HashMap::new();
  let mut person = HashMap::new();

  person.insert("first_name", "Eko");
  person.insert("last_name", "Khannedy");
  data.insert("person", person);

  let rendered = handlebars.render("hello", &data).unwrap();
  assert_eq!(rendered, "Hello, Eko Khannedy");
}

// ## HTML escape
#[test]
fn test_html_escape() {
  let mut handlebars = handlebars::Handlebars::new();

  handlebars
    .register_template_string("hello", "Hello, {{{name}}}")
    .unwrap();

  let mut data = std::collections::HashMap::new();
  data.insert("name", "<p>Anne</p>");

  let rendered = handlebars.render("hello", &data).unwrap();
  assert_eq!(rendered, "Hello, <p>Anne</p>");
}

//  ## Template File

#[test]
fn test_template_file() {
  let mut handlebars = Handlebars::new();

  handlebars
    .register_template_file("hello", "templates/hello.mustache")
    .unwrap();

  let mut data = HashMap::new();

  data.insert("name", "Eko");

  let rendered = handlebars.render("hello", &data).unwrap();

  assert_eq!(rendered, "Hello Eko");
}

// ## WITH

#[test]
fn test_with() {
  let mut handlebars = Handlebars::new();

  handlebars
    .register_template_file("hello", "templates/with-hello.mustache")
    .unwrap();

  let mut data = HashMap::new();

  let mut person = HashMap::new();
  person.insert("first_name", "Eko");
  person.insert("last_name", "Ayub");

  data.insert("person", person);

  let rendered = handlebars.render("hello", &data).unwrap();
  assert_eq!(rendered.contains("<h1>Hello Eko Ayub</h1>"), true);
}

// ## Serde
// `cargo add serde --features derive`
// `cargo add serde_json`

#[derive(serde::Serialize)]
struct Person {
  first_name: String,
  last_name: String,
}

#[derive(serde::Serialize)]
struct Data {
  person: Person,
}

#[test]
fn test_serde() {
  let mut handlebars = Handlebars::new();

  handlebars
    .register_template_file("hello", "templates/with-hello.mustache")
    .unwrap();

  let mut data = Data {
    person: Person {
      first_name: "Eko".to_string(),
      last_name: "Ayub".to_string(),
    },
  };

  let rendered = handlebars.render("hello", &data).unwrap();
  assert_eq!(rendered.contains("<h1>Hello Eko Ayub</h1>"), true);
}

#[test]
fn test_serde_json() {
  let mut handlebars = Handlebars::new();

  handlebars
    .register_template_file("hello", "templates/with-hello.mustache")
    .unwrap();

  let data = serde_json::json!( {
    "person": {
      "first_name": "Eko".to_string(),
      "last_name": "Ayub".to_string(),
    },
  });

  let rendered = handlebars.render("hello", &data).unwrap();
  assert_eq!(rendered.contains("<h1>Hello Eko Ayub</h1>"), true);
}

#[test]
fn test_if() {
  let mut handlebars = Handlebars::new();

  handlebars
    .register_template_file("blog", "templates/blog.mustache")
    .unwrap();

  let data = serde_json::json!( {
    "title": "Belajar rust",
    "content": "Content rust"
  });

  let rendered = handlebars.render("blog", &data).unwrap();
  assert_eq!(rendered.contains("Belajar rust"), true);
  assert_eq!(rendered.contains("Content rust"), true);
  assert_eq!(rendered.contains("Anonymous"), true);
}
#[test]
fn test_if_2() {
  let mut handlebars = Handlebars::new();

  handlebars
    .register_template_file("blog", "templates/blog.mustache")
    .unwrap();

  let data = serde_json::json!( {
    "title": "Belajar rust",
    "content": "Content rust",
    "author":"Meiau"
  });

  let rendered = handlebars.render("blog", &data).unwrap();
  assert_eq!(rendered.contains("Belajar rust"), true);
  assert_eq!(rendered.contains("Content rust"), true);
  assert_eq!(rendered.contains("Meiau"), true);
}
