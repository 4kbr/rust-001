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

// ## Tipe data Scalar

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

/* Operator
numeric operators:
+ = penjumlahan
- = pengurangan
* = perkalian
/ = pembagian
% = modulus (sisa bagi)

// di rust bisa augmented assignment seperti di javascript
Numeric Operator    |   Augmented Assignment
a = a + 100  |   a += 100
a = a - 100  |   a -= 100
a = a * 100  |   a *= 100
a = a / 100  |   a /= 100
a = a % 100  |   a %= 100
*/

#[test]
fn numeric_operator() {
    let a = 10;
    let b = 3;

    let addition = a + b;
    println!("Addition: {} + {} = {}", a, b, addition);

    let subtraction = a - b;
    println!("Subtraction: {} - {} = {}", a, b, subtraction);

    let multiplication = a * b;
    println!("Multiplication: {} * {} = {}", a, b, multiplication);

    let division = a / b;
    println!("Division: {} / {} = {}", a, b, division);

    let modulus = a % b;
    println!("Modulus: {} % {} = {}", a, b, modulus);
}

#[test]
fn augmented_assignment() {
    let mut a = 10;
    let mut b = 3;

    a += 5;
    println!("Augmented Assignment (a += 5): {}", a);

    b *= 2;
    println!("Augmented Assignment (b *= 2): {}", b);
}

#[test]
fn boolean() {
    // non explicit
    let a = true;
    // explicit
    let b: bool = false;

    println!("a = {}, b = {}", a, b);
}

/* Comparison Operators
Operator    |   Keterangan
==          |   sama dengan
!=          |   tidak sama dengan
>           |   lebih besar dari
<           |   lebih kecil dari
>=          |   lebih besar dari atau sama dengan
<=          |   lebih kecil dari atau sama dengan

// hasil dari comparison operator adalah boolean (true / false)
*/

#[test]
fn comparison() {
    let a = 20;
    let b = 21;

    let result: bool = a >= b;
    println!("Hasil dari {} >= {} adalah {}", a, b, result);
}

/* Boolean Operator
Operator    |   Keterangan
&&          |   AND (dan)
||          |   OR (atau)
!           |   NOT (bukan)

!!! ingat hanya `true` jika semua kondisi true, selain itu false
// hasil dari boolean operator adalah boolean (true / false)
*/
#[test]
fn boolean_operator() {
    let absen = 75;
    let nilai_akhir = 800;

    let lulus_absen: bool = absen >= 75;
    let lulus_nilai: bool = nilai_akhir >= 700;

    let lulus: bool = lulus_absen && lulus_nilai;

    println!("Lulus: {}", lulus);
}

/*Char
- char adalah tipe data karakter
- di rust char diapit dengan tanda kutip tunggal ('a', 'b', 'c')
*/
#[test]
fn char_type() {
    let char1: char = 'A';
    // let char2 = 'KA'; //if you meant to write a string literal, use double quotes: `"`, `"`rustc
    let char2: char = 'K';

    println!("char1: {}, char2: {}", char1, char2);
}

// ## Tipe data Compound

/* Tuple
- tuple bisa menyimpan lebih dari satu tipe data
- tipe-nya bisa berbeda-beda
- jumlah data di tuple sudah final, artinya tidak bis ditambah atau dikurangi
- cara buat tuple adalah dengan () tanda kurung
let contoh_tuple : (i32, f64, char) = (10, 3.14, 'A');

// kalau tipe datanya sama lebih baik pakai array

secara default tuple bersifat immutable, tapi bisa di buat mutable dengan kata kunci mut
let mut contoh_tuple : (i32, f64, char) = (10, 3.14, 'A');
contoh_tuple.0 = 20; // mengubah nilai pertama di tuple
*/

#[test]
fn tuple() {
    // explicit
    let data: (i32, f64, &str) = (100, 10.3, "Hello Tuple");
    println!("data tuple: {:?}", data);

    // non explicit
    let data2 = (200, 20.4, "Hello non explicit");

    // bisa di pakai
    let a = data2.0;
    let b = data2.1;
    let c = data2.2;
    let d = data2.2;

    println!("data2 tuple: a = {}, b = {}, c = {}, d = {d}", a, b, c);
}

#[test]
fn desctructuring_tuple() {
    let tuple = ("a", "b", "c", 10, 20, 30, 50.0);
    println!("tuple: {:?}", tuple);

    let (a, b, c, _d, _e, _f, _) = tuple;
    println!("a = {}, b = {}, c = {}", a, b, c);
}

#[test]
fn mutable_tuple() {
    let mut contoh_tuple: (i32, f64, char) = (10, 3.14, 'A');
    println!("contoh_tuple sebelum diubah: {:?}", contoh_tuple);

    contoh_tuple.0 = 20; // mengubah nilai pertama di tuple   
    println!("contoh_tuple setelah diubah: {:?}", contoh_tuple);
}

/* Unit
- unit adalah tuple tanpa nilai apapun, ditulisnya ()
- mungkin terlihat tidak berguna
- biasanya unit ini digunakan untuk function yang tidak membutuhkan hasil data apapun / return value
*/
fn unit() {
    println!("Hello unit")
}
#[test]
fn test_unit() {
    // let result = unit();
    let result: () = unit();

    println!("result unit: {:?}", result);

    let test: () = ();
    println!("test: {:?}", test);
}

/* Array
- bedanya array dengan tuple adalah array hanya bisa menyimpan jenis / type data yang sama
- cara membuat array adalah dengan [] tanda kurung siku
let contoh_array: [i32; 4] = [10,20,30,40];
[i32;4] artinya array yang menyimpan data bertipe i32 dengan jumlah 4 elemen

- cara akses elemen diarray menggunakan [index] mirip seperti di javascript

- secara default array adalah immutable
- bisa di buat mutable dengan kata kunci mut
let mut contoh_array: [i32; 4] = [10,20,30,40];
contoh_array[0] = 100; // mengubah elemen pertama di array

- get jumlah data di array dengan .len()

## Two dimensional array
- kita bisa menyimpan array didalam array
let two_d_array: [[i32;3];2] = [
    [1,2,3],
    [4,5,6]
];
two_d_array[0][1] // mengakses elemen baris pertama kolom kedua (value 2)
*/
#[test]
fn array() {
    // explicit
    let array_explicit: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array explicit: {:?}", array_explicit);
    // non explicit
    let array_non_explicit = [6, 7, 8, 9, 10];
    println!("array non_explicit: {:?}", array_non_explicit);

    // akses elemen di array
    let first = array_explicit[0];
    println!("first array: {first}");

    // mutable array
    let mut mutable_array = [10, 20, 30, 40, 50];
    mutable_array[0] = 100; // mengubah elemen pertama di array
    println!("mutable_array setelah diubah: {:?}", mutable_array);

    // panjang array
    let length = mutable_array.len();
    println!("panjang mutable_array: {}", length);
}

#[test]
fn two_dimensional_array() {
    //  kita bisa menyimpan array didalam array
    let two_d_array: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];
    println!("two_d_array: {:?}", two_d_array);
    println!("two_d_array[0][1]: {}", two_d_array[0][1]); // mengakses elemen baris pertama kolom kedua (value 2)
}

/* Constant
- constant bersifat immutable
- harus di assign type dan nilai saat deklarasi
- biasanya pakai huruf besar semua
- cocok untuk menyimpan nilai yang tidak berubah sepanjang program berjalan
*/

const MAXIMUM: i32 = 100;
#[test]
fn constant() {
    //  const MINIMUM = 0; //missing type for `const`
    const MINIMUM: i32 = 0; //missing type for `const`

    println!("MAXIMUM: {}, MINIMUM: {}", MAXIMUM, MINIMUM);
}

/* Variable scope
- variable yang dibuat di dalam blok {} hanya bisa diakses di dalam blok tersebut, jika diakses diluar akan error
- sangat berhubungan dengan manajemen memory di rust
*/

#[test]
fn variable_scope() {
    let outer_var = "I'm outside!";
    {
        let inner_var = "I'm inside!";
        println!("inner_var: {}", inner_var);
        println!("outer_var from inside: {}", outer_var);
    }
    // println!("inner_var: {}",inner_var) // the binding `inner_var` is available in a different scope in the same function
}

/* Garbage Collection
- Garbage Collection adalah fitur yang banyak digunakan bahasa pemrograman untuk melakukan manajemen memory, seperti Java dan Golang
- Secara berkala Garbage Collection akan memantau data yang sudah tidak digunakan lagi di memory, dan menghapusnya secara otomatis
- Atau di bahasa pemrograman tanpa Garbage Collection, yang biasanya harus melakukan manajemen memory secara manual, seperti C/C++
- Tanpa Garbage Collection, kita harus mengalokasikan data secara manual di memory, begitu juga ketika sudah tidak butuh, kita harus menghapus data dari memory dari memory

- Rust memiliki pendekatan yang berbeda, Rust tidak menggunakan Garbage Collection, Rust juga tidak menggunakan Manual Memory Management
*/

/* Stack dan Heap
- Rust membagi data di memory dalam dua bagian, Stack dan Heap
- Stack adalah bagian dimana data disimpan dalam struktur data tumpukan, last in first out. Semua data di Stack harus yang fixed size (artinya ukuran data sudah pasti)
- Heap berbeda, heap seperti tempat untuk menyimpan data, dimana untuk menyimpan data di Heap kita akan melakukan request ke Heap, lalu di dalam Heap terdapat Memory Allocator yang bertugas untuk menemukan area kosong untuk menyimpan dan mengalokasikan data ke area tersebut. Setelah itu kita akan diberi pointer (penunjuk) ke lokasi dimana data itu berada di Heap.
- Pointer dari Heap berukuran fix sized, oleh karena itu pointer akan disimpan di Stack
- Data yang disimpan di Heap bisa berukuran dinamis (dynamic sized), artinya ukuran data bisa berubah-ubah selama program berjalan

- Contoh data yang disimpan di Stack adalah tipe data scalar seperti integer, float, boolean, char, dan juga compound seperti tuple dengan ukuran tetap
- Contoh data yang disimpan di Heap adalah tipe data String, Vector, dan juga tipe data lain yang ukurannya bisa berubah-ubah

*/

/* Drop Function

- Saat variable keluar dari scope nya, yang artinya tidak bisa diakses lagi, secara otomatis Rust akan memanggil drop function
- Drop function adalah function untuk menghapus data, sehingga akan dibersihkan dari Heap
- Dan jika Rust function() sudah selesai dieksekusi, maka function() tersebut akan dihapus pula dari Stack Frame
- Oleh karena itu, Rust tidak membutuhkan Garbage Collection ataupun Manual Memory Management

*/
#[test]
fn stack_heap() {
    function_a();
    // setelah function a dipanggil selesai, semua data di function a akan dihapus dari stack

    function_b();
    // setelah function b dipanggil selesai, semua data di function b akan dihapus dari stack

    // kalau pakai garbage collection, data baru dihapus sampai garbage collection-nya memanggil functionnya secara berkala
    // kalau di manual memory management, kita harus menghapus data secara manual
    // kalau dirust data akan dihapus setelah out of scope atau keluar dari scope-nya
}
// function disimpan di stack
pub fn function_a() {
    // ini disimpan di stack
    let a = 10;
    // ini disimpan di heap
    let b = String::from("value function a");

    println!("function_a: a = {}, b = {}", a, b);
}

// function disimpan di stack
pub fn function_b() {
    // ini disimpan di stack
    let a = 10;
    // ini disimpan di heap
    let b = String::from("value function b");
    println!("function_b: a = {}, b = {}", a, b);
}

/*
Manajemen memori adalah proses pengelolaan alokasi dan dealokasi memori untuk program yang berjalan. Rust memiliki model kepemilikan (ownership) yang unik untuk menghindari masalah umum seperti kebocoran memori dan data race.

1. **Ownership**: Setiap nilai memiliki pemilik. Saat pemilik dihapus, nilai tersebut juga dihapus dari memori.
2. **Borrowing**: Rust memungkinkan referensi (borrow) ke nilai tanpa mengambil kepemilikan. Borrowing ini bisa bersifat mutable atau immutable.
3. **Lifetime**: Rust mengelola berapa lama referensi dapat bertahan dalam program, sehingga mencegah penggunaan referensi yang tidak valid.

Model ini memungkinkan Rust untuk menjalankan kode yang aman secara memori tanpa garbage collector, memastikan kinerja dan keamanan.
*/

// Fungsi untuk mengembalikan nama
pub fn get_name() -> String {
    String::from("Rust")
}

// Fungsi yang meminjam nama (borrow)
pub fn print_name(name: &String) {
    println!("Name is: {}", name);
}

#[test]
fn test_memory_management() {
    let my_name = get_name(); // Ownership berpindah ke my_name
    print_name(&my_name); // Borrow my_name sebagai referensi
    // my_name masih dapat digunakan di sini karena tidak diambil sendiri
    println!("My name again is: {}", my_name);
}
