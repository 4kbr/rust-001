/*
# Configuration di rust pakai crate / package config

Menambah Library

cargo add serde --features derive
cargo add config

*/

use config::Config;

fn main() {
    println!("Hello, world!");
}

// ## setup
#[test]
fn test_config() {
    let config: Config = Config::builder().build().unwrap();

    assert!(config.get_string("APP_NAME").is_err());
}

// ## Environment variable
#[test]
fn test_config_env() {
    use std::env::set_var;
    unsafe {
        set_var("DB_HOST", "localhost");
        set_var("DB_PORT", "5432");
        set_var("DB_USER", "eko");
        set_var("DB_PASSWORD", "rahasia");
    }

    let config = Config::builder()
        .add_source(config::Environment::default().convert_case(config::Case::Snake))
        .build()
        .unwrap();

    println!("db host: {}", config.get_string("db_host").unwrap());
    assert_eq!(config.get_string("db_host").unwrap(), "localhost");
    assert_eq!(config.get_int("db_port").unwrap(), 5432);

    assert_eq!(config.get_string("db_user").unwrap(), "eko");
    assert_eq!(config.get_string("db_password").unwrap(), "rahasia");
}
