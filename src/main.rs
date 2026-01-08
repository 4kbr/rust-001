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
}
