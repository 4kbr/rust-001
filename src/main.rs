use std::collections::HashMap;

use handlebars::Handlebars;

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

// ## escape
#[test]
fn test_html_escape() {
    let mut handlebars = handlebars::Handlebars::new();

    handlebars
        .register_template_string("hello", "Hello, {{{name}}}")
        .unwrap();

    handlebars
        .register_template_string("bye", "Bye, {{name}}")
        .unwrap();

    let mut data = std::collections::HashMap::new();
    data.insert("name", "<p>Eko</p>");
    let rendered = handlebars.render("hello", &data).unwrap();
    assert_eq!(rendered, "Hello, <p>Eko</p>");
    let rendered = handlebars.render("bye", &data).unwrap();
    assert_eq!(rendered, "Bye, Eko");
}
