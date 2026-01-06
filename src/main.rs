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
struct Address {
  street: String,
  city: String,
}
#[derive(serde::Serialize)]
struct Person {
  first_name: String,
  last_name: String,
  hobbies: Vec<String>,
  addresses: Vec<Address>,
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
      hobbies: vec![],
      addresses: vec![],
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

// ## Unless
#[test]
fn test_unless() {
  let mut handlebars = Handlebars::new();

  handlebars
    .register_template_file("footer", "templates/footer.mustache")
    .unwrap();

  let data = json!({});

  let rendered = handlebars.render("footer", &data).unwrap();
  assert_eq!(
    rendered.contains("This content does not contains footer"),
    true
  );
}

#[test]
fn test_unless_2() {
  let mut handlebars = Handlebars::new();

  handlebars
    .register_template_file("footer", "templates/footer.mustache")
    .unwrap();

  let data = json!({
      "footer" : "Siap"
  });

  let rendered = handlebars.render("footer", &data).unwrap();
  assert_eq!(
    rendered.contains("This content does not contains footer"),
    false
  );
}

#[test]
fn test_each() {
  let mut handlebars = Handlebars::new();

  handlebars
    .register_template_file("person", "templates/person.mustache")
    .unwrap();

  let data = Person {
    first_name: "Eko".to_string(),
    last_name: "Sutton".to_string(),
    hobbies: vec!["Coding".to_string(), "Gaming".to_string()],
    addresses: vec![],
  };

  let rendered = handlebars.render("person", &data).unwrap();
  assert_eq!(rendered.contains("Eko"), true);
  assert_eq!(rendered.contains("Sutton"), true);
  assert_eq!(rendered.contains("0 - Coding"), true);
  assert_eq!(rendered.contains("1 - Gaming"), true);
}

#[test]
fn test_each_object() {
  let mut handlebars = Handlebars::new();

  handlebars
    .register_template_file("person", "templates/person.mustache")
    .unwrap();

  let data = Person {
    first_name: "Eko".to_string(),
    last_name: "Hale".to_string(),
    hobbies: vec!["Coding".to_string(), "Gaming".to_string()],
    addresses: vec![
      Address {
        street: "Jl. Kampung 1".to_string(),
        city: "Jakarta".to_string(),
      },
      Address {
        street: "Jl. Kampung 2".to_string(),
        city: "Bandung".to_string(),
      },
    ],
  };

  let rendered = handlebars.render("person", &data).unwrap();
  println!("{}", rendered);

  assert_eq!(rendered.contains("Eko"), true);
  assert_eq!(rendered.contains("Hale"), true);
  assert_eq!(rendered.contains("0 - Coding"), true);
  assert_eq!(rendered.contains("1 - Gaming"), true);
  assert_eq!(rendered.contains("street - Jl. Kampung 1"), true);
  assert_eq!(rendered.contains("city - Jakarta"), true);
  assert_eq!(rendered.contains("street - Jl. Kampung 2"), true);
  assert_eq!(rendered.contains("city - Bandung"), true);
}

struct DoubleNumber;

impl handlebars::HelperDef for DoubleNumber {
  fn call<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'rc>,
    r: &'reg Handlebars<'reg>,
    ctx: &'rc handlebars::Context,
    rc: &mut handlebars::RenderContext<'reg, 'rc>,
    out: &mut dyn handlebars::Output,
  ) -> handlebars::HelperResult {
    let param = h.param(0).unwrap();
    let number = param.value().as_i64().unwrap();
    out.write(&format!("{}", number * 2))?;
    Ok(())
  }
}

#[test]
fn test_helper() {
  let mut handlebars = Handlebars::new();
  handlebars.register_helper("double", Box::new(DoubleNumber));
  handlebars
    .register_template_string("helper", "Result : {{double value}}")
    .unwrap();

  let data = json!({
      "value" : 20
  });

  let rendered = handlebars.render("helper", &data).unwrap();
  assert_eq!(rendered.contains("Result : 40"), true);
}

handlebars::handlebars_helper!(uppercase: |value: String| {
  value.to_uppercase()
});

#[test]
fn test_helper_macro() {
  let mut handlebars = Handlebars::new();
  handlebars.register_helper("uppercase", Box::new(uppercase));
  handlebars
    .register_template_string("helper", "Hello : {{uppercase name}}")
    .unwrap();

  let data = json!({
      "name" : "Eko"
  });

  let rendered = handlebars.render("helper", &data).unwrap();
  assert_eq!(rendered.contains("Hello : EKO"), true);
}

#[test]
fn test_partial() {
  let mut handlebars = Handlebars::new();

  handlebars
    .register_template_file("layout/header", "templates/layout/header.mustache")
    .unwrap();
  handlebars
    .register_template_file("layout/footer", "templates/layout/footer.mustache")
    .unwrap();
  handlebars
    .register_template_file("hello", "templates/blog.mustache")
    .unwrap();

  let data = json!({
      "title" : "Belajar Rust",
      "content" : "Belajar Rust dengan baik",
      "footer" : "Siap Oke"
  });

  let rendered = handlebars.render("hello", &data).unwrap();
  println!("{}", rendered);

  assert_eq!(rendered.contains("Belajar Rust"), true);
  assert_eq!(rendered.contains("Belajar Rust dengan baik"), true);
  assert_eq!(rendered.contains("Anonymous"), true);
  assert_eq!(rendered.contains("Siap Oke"), true);
}
