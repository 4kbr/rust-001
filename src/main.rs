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
    use log;

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
}
