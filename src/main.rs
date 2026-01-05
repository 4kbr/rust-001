use handlebars::Handlebars;

// ## TEMPLATE
// pakai library `handlebars`
// `cargo add handlebars`
fn main() {
    println!("Hello, world!");
}

// ## Setup
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
