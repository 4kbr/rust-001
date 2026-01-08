/*
# Rust Logging
- Rust memiliki fitur untuk logging yang dibuat dalam Crate Log
- https://crates.io/crates/log
- Namun Crate Log tersebut hanyalah kontrak untuk melakukan logging, untuk implementasinya sendiri kita perlu memilih Crate lainnya
- Salah satu keuntungan menggunakan kontrak adalah, kita bisa berganti-ganti implementasi, tanpa harus mengubah kode logging nya
*/

// `cargo add log`

/*
# Melakukan Log
- Untuk melakukan log, kita bisa menggunakan macro log! (level, log)
- https://docs.rs/log/0.4.22/log/macro.log.html
- Atau kita bisa gunakan shortcut macro
- error!(log) untuk level error
- warn!(log) untuk level warn
- info!(log) untuk level info
- debug!(log) untuk level debug
- trace!(log) untuk level trace
- https://docs.rs/log/0.4.22/log/index.html#macros
*/
fn main() {
    println!("Hello, world!");
}
#[cfg(test)]
mod tests {
    use log::{self, debug, error, info, trace, warn};

    // test kode level
    #[test]
    fn it_works() {
        log::error!("This is a error");
        log::warn!("This is a warn");
        log::info!("This is a info");
        log::debug!("This is a debug");
        log::trace!("This is a trace");
    }
    // ## env logger
    /*
    # Simple Logger
    - Karena Crate Log hanya kontrak, maka kita harus pilih implementasinya
    - Salah satu implementasi yang sederhana adalah Env Logger
    - Env Logger bisa digunakan untuk menampilkan log ke Console / Terminal, dan level yang akan diaktifkan bisa di set via Env Variable sistem operasi
    - https://crates.io/crates/env_logger

    cargo add env_logger
    */
    #[test]
    fn test_env_logger() {
        env_logger::init();

        log::error!("This is a error");
        log::warn!("This is a warn");
        log::info!("This is a info");
        log::debug!("This is a debug");
        log::trace!("This is a trace");

        // export RUST_LOG=info
        // echo $RUST_LOG

        // [2026-01-08T15:25:50Z ERROR logging::tests] This is a error
        // [2026-01-08T15:25:50Z WARN  logging::tests] This is a warn
        // [2026-01-08T15:25:50Z INFO  logging::tests] This is a info
    }

    /*
    # Complex Logger

    - Env Logger hanya bisa digunakan untuk menampilkan log ke Console, bagaimana jika kita ingin menampilkan log ke tempat lain? Misal ke file
    - Atau mengatur level tergantung module nya?
    - Kita bisa menggunakan implementasi Logger yang lebih kompleks, contohnya adalah log4rs
    - https://crates.io/crates/log4rs

    `cargo add log4rs`

    # Configuration
    - Untuk menggunakan Log4rs, kita bisa menyimpan semua konfigurasinya menggunakan file konfigurasi yaml
    - Selanjutnya kita bisa baca file konfigurasi yaml tersebut menggunakan library Log4rs

    */
    // test complex logger
    #[test]
    fn test_log4rs() {
        log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

        error!("This is a error");
        warn!("This is a warning");
        info!("This is a info");
        debug!("This is a debug");
        trace!("This is a trace");

        // 2026-01-09T06:30:09.178815656+01:00 ERROR logging::tests - This is a error
        // 2026-01-09T06:30:09.178919878+01:00 WARN logging::tests - This is a warning
        // 2026-01-09T06:30:09.178933056+01:00 INFO logging::tests - This is a info

        // 2026-01-09T06:35:47.235837550+01:00 ERROR logging::tests - This is a error
        // 2026-01-09T06:35:47.235924467+01:00 WARN logging::tests - This is a warning
        // 2026-01-09T06:35:47.235940911+01:00 INFO logging::tests - This is a info
        // 2026-01-09T06:35:47.235955747+01:00 DEBUG logging::tests - This is a debug
        // 2026-01-09T06:35:47.235970474+01:00 TRACE logging::tests - This is a trace
    }
    /*
    # Loggers
    - Salah satu kelebihan Log4rs adalah, kita bisa mudah mengubah Level untuk module-module tanpa harus mengubah kode program
    - Kita hanya perlu mengubah file konfigurasinya saja
    - https://docs.rs/log4rs/1.3.0/log4rs/config/index.html#loggers
    */
}

#[cfg(test)]
mod tests2 {
    use log::{self, debug, error, info, trace, warn};

    // test kode level
    #[test]
    fn it_works() {
        log::error!("This is a error");
        log::warn!("This is a warn");
        log::info!("This is a info");
        log::debug!("This is a debug");
        log::trace!("This is a trace");
    }
    // ## env logger
    /*
    # Simple Logger
    - Karena Crate Log hanya kontrak, maka kita harus pilih implementasinya
    - Salah satu implementasi yang sederhana adalah Env Logger
    - Env Logger bisa digunakan untuk menampilkan log ke Console / Terminal, dan level yang akan diaktifkan bisa di set via Env Variable sistem operasi
    - https://crates.io/crates/env_logger

    cargo add env_logger
    */
    #[test]
    fn test_env_logger() {
        env_logger::init();

        log::error!("This is a error");
        log::warn!("This is a warn");
        log::info!("This is a info");
        log::debug!("This is a debug");
        log::trace!("This is a trace");

        // export RUST_LOG=info
        // echo $RUST_LOG

        // [2026-01-08T15:25:50Z ERROR logging::tests] This is a error
        // [2026-01-08T15:25:50Z WARN  logging::tests] This is a warn
        // [2026-01-08T15:25:50Z INFO  logging::tests] This is a info
    }

    /*
    # Complex Logger

    - Env Logger hanya bisa digunakan untuk menampilkan log ke Console, bagaimana jika kita ingin menampilkan log ke tempat lain? Misal ke file
    - Atau mengatur level tergantung module nya?
    - Kita bisa menggunakan implementasi Logger yang lebih kompleks, contohnya adalah log4rs
    - https://crates.io/crates/log4rs

    `cargo add log4rs`

    # Configuration
    - Untuk menggunakan Log4rs, kita bisa menyimpan semua konfigurasinya menggunakan file konfigurasi yaml
    - Selanjutnya kita bisa baca file konfigurasi yaml tersebut menggunakan library Log4rs

    */
    // test complex logger
    #[test]
    fn test_log4rs() {
        log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

        error!("This is a error");
        warn!("This is a warning");
        info!("This is a info");
        debug!("This is a debug");
        trace!("This is a trace");

        // 2026-01-09T06:30:09.178815656+01:00 ERROR logging::tests - This is a error
        // 2026-01-09T06:30:09.178919878+01:00 WARN logging::tests - This is a warning
    }
    /*
    # Loggers
    - Salah satu kelebihan Log4rs adalah, kita bisa mudah mengubah Level untuk module-module tanpa harus mengubah kode program
    - Kita hanya perlu mengubah file konfigurasinya saja
    - https://docs.rs/log4rs/1.3.0/log4rs/config/index.html#loggers
    */
}
