/*
# Rust Logging
- Rust memiliki fitur untuk logging yang dibuat dalam Crate Log
- https://crates.io/crates/log
- Namun Crate Log tersebut hanyalah kontrak untuk melakukan logging, untuk implementasinya sendiri kita perlu memilih Crate lainnya
- Salah satu keuntungan menggunakan kontrak adalah, kita bisa berganti-ganti implementasi, tanpa harus mengubah kode logging nya
*/

// `cargo add log`
fn main() {
    println!("Hello, world!");
}
