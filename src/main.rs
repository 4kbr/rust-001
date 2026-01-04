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
