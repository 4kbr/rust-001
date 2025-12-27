fn main() {
    // ```cargo run```
    //  ```cargo run``` untuk menjalankan rust dengan membuat file bin di folder target/debug
    //   ```cargo build --release``` untuk membuat bin
    println!("Hello, world!");
    // let x:i32 = 1;
}

// ```cargo test hello_test -- --exact```
// ```cargo test hello_test -- --exact --nocapture```
#[test]
fn hello_test() {
    println!("hello_test: called")
}

// variable bersifat immutable (tidak bisa diubah) secara default
#[test]
fn test_variable() {
    let name = "My Name";
    // name = "sada"; // error
    println!("hello {}", name);
}

// variable dengan kata kunci key mut = bisa diubah / assign ulang
#[test]
fn test_mutable() {
    let mut mutable_name = "My Name";
    println!("hello before edit {}", mutable_name);

    mutable_name = "Antohr";
    println!("hello after edit {}", mutable_name);
}

// rust bersofat static typing, jadi variable yang sudah diassign dengan type data tertentu tidak bisa diubah kembali type datanya
#[test]
fn static_typing() {
    // let mut name = "My Name";
    let mut name = "My Name";
    println!("hello {}", name);

    // name = 10; // ini gak bisa
    name = "10";
    println!("hello {}", name);
}

#[test]
fn data_type() {
    // setiap nilai di rust punya type data, secara garis besar type data di rust dibagi 2 yaitu "scalar" dan "compound"
    // scalar type yang single value
    // seperti `interger`, `float`, `boolean`, dan `char`.
    //  misal `let my_age = 10`

    // sementara compound type mempresentasikan beberapa value (mirip seperti object dan array di javascript),
    // yaitu: `tuple` dan `array`
    // let first_array:[i32;4] = [1,23,4,2];

    // secara default rust akan mendeteksi type data yang kita pakai, tanpa perlu diketik sendiri, tapi juga bisa diketik sendiri

    // misal explicit
    let age: i32 = 20;
    println!("umurnya adalah: {}", age); // bisa seperti ini
    // println!("umurnya adalah: {age}");

    // misal non explicit, rust auto assign
    let age_no_explicit = 20;
    println!("umurnya adalah: {age_no_explicit}");
}

/* Integer type
Panjang  |   Signed  |   Unsigned
8 bit    |   i8      |   u8
16 bit   |   i16     |   u16
32 bit   |   i32     |   u32    <- (default variable number kalau tidak di assign) `let age = 40` maka age: i32
64 bit   |   i64     |   u64
128 bit  |   i128    |   u128

// bedanya i(n) dengan u(n) adalah u hanya bisa value yang bernilai positif (+) dan tidak bisa menyimpan value negatif (-)
// !!! Dan ingat selalu hati-hati saat mengconversi dari tipe data yang lebih besar ke yang lebih kecil, karena bisa menyebabkan overflow
// misalnya mengubah i32 ke i8, a:i32 = 300, kalau diubah ke i8 akan overflow karena i8 hanya bisa menyimpan nilai -128 sampai 127
// overflow tidak akan menyebabkan error syntax tapi akan menyebabkan nilai random yang tidak terduga
*/

/* Float
Panjang |   Float
32-bit  |   f32
64-bit  |   f64     <- (default float yang dipilih) `let height = 170.0` maka height: f64
*/

/* Usize
// Usize adalah tipe data number integer yang panjangnya mengikuti platform (sistem operasi), misal device-nya 32bit atau 64 bit

Usize   |   Keterangan
isize   |   32-bit / 64-bit
usize   |   32-bit / 64-bit (valuenya tidak boleh negatif)
*/
#[test]
fn number() {
    let ai8: i8 = 127; //  range is `-128..=127`
    println!("ai8 = {ai8}");
    let ai16: i16 = 32767; // range is `-32768..=32767
    println!("ai16 = {ai16}");
    let ai32: i32 = 2147483647; // range is `-2147483648..=2147483647`
    println!("ai32 = {ai32}");
    let ai64: i64 = 9223372036854775807; // range is `-9223372036854775808..=9223372036854775807`
    println!("ai64 = {ai64}");

    let af32: f32 = 3.14; // range is `±1.18e-38 to ±3.4e38` with 6 decimal digits precision
    println!("af32 = {af32}");
    let af64: f64 = 2.718281828459045; // range is `±2.23e-308 to ±1.79e308` with 15 decimal digits precision
    println!("af64 = {af64}");
}

pub fn int_to_float(value: i32) -> f64 {
    value as f64
}

pub fn float_to_int(value: f64) -> i32 {
    value as i32
}

#[test]
fn test_int_to_float() {
    let int_value: i32 = 10;
    let float_value = int_to_float(int_value);
    assert_eq!(float_value, 10.0);
}

#[test]
fn test_float_to_int() {
    let float_value: f64 = 10.5;
    let int_value = float_to_int(float_value);
    assert_eq!(int_value, 10);
}

#[test]
fn test_large_to_small_int_conversion() {
    let large_int: i64 = 1_000_000_000;
    let small_int: i32 = large_int as i32; // Mengubah dari i64 ke i32
    assert_eq!(small_int, 1_000_000_000i32); // Hati-hati dengan overflow
}
#[test]
fn test_large_to_small_int_conversion_overflow() {
    let large_int: i64 = 1_000_000_000_000;
    let small_int: i32 = large_int as i32; // i32 tidak akan bisa menampung nilai sebesar ini, sehingga valuenya akan random karena overflow
    println!("small_int: {}", small_int); // small_int: -727379968
}

#[test]
fn test_small_to_large_int_conversion() {
    let small_int: i32 = 123456;
    let large_int: i64 = small_int as i64; // Mengubah dari i32 ke i64
    assert_eq!(large_int, 123456);
}
