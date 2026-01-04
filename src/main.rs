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

#[test]
fn test_json() {
    let config = Config::builder()
        .add_source(config::File::new(
            "application.json",
            config::FileFormat::Json,
        ))
        .build()
        .unwrap();

    assert_eq!(config.get_string("name").unwrap(), "My Application");
    assert_eq!(config.get_string("database.host").unwrap(), "localhost");

    assert_eq!(config.get_int("database.port").unwrap(), 5432);

    assert_eq!(config.get_string("database.name").unwrap(), "my_database");

    assert_eq!(config.get_string("database.username").unwrap(), "user");

    assert_eq!(config.get_string("database.password").unwrap(), "password");
}
#[test]
fn test_yaml() {
    let config = Config::builder()
        .add_source(config::File::new(
            "application.yaml",
            config::FileFormat::Yaml,
        ))
        .build()
        .unwrap();

    assert_eq!(config.get_string("name").unwrap(), "My Application");
    assert_eq!(config.get_string("database.host").unwrap(), "localhost");

    assert_eq!(config.get_int("database.port").unwrap(), 5432);

    assert_eq!(config.get_string("database.name").unwrap(), "my_database");

    assert_eq!(config.get_string("database.username").unwrap(), "user");

    assert_eq!(config.get_string("database.password").unwrap(), "password");
}
#[test]
fn test_toml() {
    let config = Config::builder()
        .add_source(config::File::new(
            "application.toml",
            config::FileFormat::Toml,
        ))
        .build()
        .unwrap();

    assert_eq!(config.get_string("name").unwrap(), "My Application");
    assert_eq!(config.get_string("database.host").unwrap(), "localhost");

    assert_eq!(config.get_int("database.port").unwrap(), 5432);

    assert_eq!(config.get_string("database.name").unwrap(), "my_database");

    assert_eq!(config.get_string("database.username").unwrap(), "user");

    assert_eq!(config.get_string("database.password").unwrap(), "password");
}

#[derive(Debug, serde::Deserialize)]
pub struct AppConfig {
    name: String,
    database: DatabaseConfig,
}
#[derive(Debug, serde::Deserialize)]
pub struct DatabaseConfig {
    host: String,
    port: i32,
    name: String,
    username: String,
    password: String,
}
#[test]
fn test_deserialization() {
    let config = Config::builder()
        .add_source(config::File::new(
            "application.toml",
            config::FileFormat::Toml,
        ))
        .build()
        .unwrap();

    let app_config: AppConfig = config.try_deserialize().unwrap();

    println!("app_config: {:?}", app_config);

    assert_eq!(app_config.name, "My Application");
    assert_eq!(app_config.database.host, "localhost");
    assert_eq!(app_config.database.port, 5432);
    assert_eq!(app_config.database.name, "my_database");
    assert_eq!(app_config.database.username, "user");
    assert_eq!(app_config.database.password, "password");
}
