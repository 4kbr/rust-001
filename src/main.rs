use std::{ops::Index, result};

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

/* &str dan String

- Rust memiliki tipe data text yang fixed size, yaitu &str (string slice), dan yang bisa mengembang ukurannya, yaitu String
- &str karena ukurannya fixed size, jadi Rust akan menyimpannya di Stack, sedangkan String karena bisa mengembang, maka disimpan di Heap
- &str biasanya digunakan untuk menyimpan text yang sudah pasti isinya, misal literal string
let greeting: &str = "Hello, world!";

- String biasa digunakan untuk menyimpan value dari luar, misal input dari user, atau data dari file, database, atau network

# Immutable str

- Karena ukuran &str adalah fixed size, maka operasi &str adalah tipe data yang immutable, artinya isi data &str tidak bisa diubah
- Ketika kita buat variable mutable, dan mengubah data &str, sebenarnya yang dilakukan adalah mengganti isi variable, bukan mengubah isi dari &str
let mut greeting: &str = "Hello, world!";
greeting = "Hello, Rust!"; // ini bukan mengubah isi dari &str, tapi mengganti isi variable greeting dengan &str yang baru
- Jadi, &str yang lama "Hello, world!" tetap ada di memory, dan tidak bisa diubah isinya

- &str memiliki banyak sekali method yang bisa digunakan untuk memanipulasi &str nya, namun akan menghasilkan nilai &str baru
- Namun perlu diperhatikan, beberapa method dari &str akan mengembalikan bentuk data String, bukan &str
- https://doc.rust-lang.org/std/primitive.str.html


# String

- String di Rust merupakan tipe data text UTF-8, dan bisa berkembang ukurannya
- Ketika kita buat dalam bentuk immutable variable, maka String tidak bisa berkembang, namun tetap disimpan di Heap
- Ketika kita buat dalam bentuk mutable variable, maka String bisa berkembang di Heap
- String juga memiliki method / function untuk memanipulasi data, namun perlu diperhatikan ada method yang digunakan untuk mengubah datanya sendiri, ada juga method yang digunakan untuk mengubah dalam bentuk data baru, tanpa memodifikasi data asli nya
- https://doc.rust-lang.org/std/string/struct.String.html

let name:String = String::from("Hello");
name.push_str(" World"); // walau function-nya benar, ini tidak bisa dilakukan karena name adalah immutable

let mut name:String = String::from("Hello");
name.push_str(" World"); // ini baru bisa dilakukan

*/

#[test]
fn string_str() {
    // disimpan di stack
    let nama_str: &str = "  Nama ku pakai     ";
    let trim: &str = nama_str.trim(); // menghapus spasi di awal dan akhir

    println!("nama_str = {} ", nama_str);
    println!("trim = {} ", trim);

    let mut username: &str = "user123";
    println!("username sebelum diubah: {}", username);

    username = "user098"; // sebenarnya ini hanya mengganti isi variable, bukan merubah value dari "user123", "user123" tetap ada dimemory
    println!("username setelah diubah: {}", username);

    // itu juga berlaku untuk integer
    let mut age: i32 = 20;
    age = 22; // ini hanya mengganti isi variable age, bukan merubah value 20, 20 tetap ada dimemory
}

#[test]
fn string_type() {
    // let name:String = String::from("Hello");
    // println!("name: {}", name);
    // name.push_str(" World"); // walau function-nya benar, ini tidak bisa dilakukan karena name adalah immutable

    // disimpan di heap
    let mut sapa: String = String::from("Hello");
    println!("sapa: {}", sapa);
    sapa.push_str(" World");

    let sapa_gaul = sapa.replace("Hello", "Whatsapp"); // .replace tidak merubah data asli, tapi mengembalikan data baru
    println!("sapa {}", sapa); // tetap "Hello World"
    println!("sapa_gaul {}", sapa_gaul); // string baru
}

/*
Manajemen memori adalah proses pengelolaan alokasi dan dealokasi memori untuk program yang berjalan. Rust memiliki model kepemilikan (ownership) yang unik untuk menghindari masalah umum seperti kebocoran memori dan data race.

1. **Ownership**: Setiap nilai memiliki pemilik. Saat pemilik dihapus, nilai tersebut juga dihapus dari memori.
2. **Borrowing**: Rust memungkinkan referensi (borrow) ke nilai tanpa mengambil kepemilikan. Borrowing ini bisa bersifat mutable atau immutable.
3. **Lifetime**: Rust mengelola berapa lama referensi dapat bertahan dalam program, sehingga mencegah penggunaan referensi yang tidak valid.

Model ini memungkinkan Rust untuk menjalankan kode yang aman secara memori tanpa garbage collector, memastikan kinerja dan keamanan.

# Ownership
- Rust menggunakan Ownership untuk melakukan data management di Memory
- Ownership adalah salah satu fitur unik di Rust yang mungkin jarang ada di bahasa pemrograman lain
- Ownership wajib dimengerti, karena akan berdampak ke hampir semua fitur di Rust
- Ownership adalah fitur yang digunakan oleh Rust untuk menjadikan Rust menjadi bahasa pemrograman yang aman dalam mengelola data di memory, tanpa harus adanya fitur Garbage Collection atau Manual Memory Management
- Karena Ownership adalah konsep yang baru untuk kebanyakan programmer, maka kadang kita butuh waktu untuk memahaminya

# Ownership Rules

- Setiap value di Rust harus punya owner (variable pemilik value)
- Dalam satu waktu, hanya boleh ada satu owner
- Ketika owner keluar scope, value akan dihapus

# Data Copy

- Sesuai aturan di Ownership Rules, setiap value harus dimiliki oleh satu owner pada satu waktu
- Ketika kita berinteraksi dengan data, maka data akan dimiliki hanya oleh satu owner
- Semua data yang bersifat fixed size (yang disimpan di Stack), ketika kita tambahkan ke variable berbeda (owner baru), maka hasilnya adalah data akan di copy, sehingga variable baru (owner baru) akan memiliki data hasil copy dari variable lama (owner lama)
- Oleh karena itu, tiap data akan selalu dimiliki oleh satu owner pada satu waktu
let a = 10;
let b = a; // data a bukan di 'move', tapi di copy
// jadi baik a maupun b bisa diakses
println!("a: {}, b: {}", a, b);


# Ownership Movement

- Namun Data Copy tidak terjadi untuk tipe data yang disimpan di Heap
- Seperti aturan di Ownership, dalam satu waktu value hanya dimiliki satu owner
- Maka ketika kita coba buat variable baru (owner baru) dari variable lama (owner lama), maka yang terjadi bukanlah copy, melainkan transfer ownership dari owner lama ke owner baru
- Setelah proses transfer selesai, secara otomatis owner lama akan dianggap tidak valid lagi digunakan
let s1 = String::from("Hello");
// ownership dari s1 pindah ke s2
let s2 = s1 // dari sini s1 tidak bisa diakses lagi
println!("s2: {}", s2);
// println!("s1: {}", s1); // akan error

# Clone

- Sekarang kita tahu bahwa data di Stack akan di Copy sedangkan data di Heap akan dipindahkan ownership nya
- Lantas bagaimana jika kita juga ingin melakukan Copy untuk data di Heap?
- Maka kita harus melakukan Clone
- Clone adalah membuat data tiruan yang sama dari data aslinya
- String memiliki method clone() untuk melakukan ini
- Saat kita memanggil method clone() maka method tersebut akan meng-copy data String menjadi data String baru
- Semua tipe data yang disimpan di Heap di Rust memiliki method clone()
let s1 = String::from("Hello");
let s2 = s1.clone(); // s1 tidak dipindahkan ownership-nya, tapi di clone
println!("s1: {}, s2: {}", s1, s2); // bisa diakses kedua-nya

*/

#[test]
fn ownership_rules() {
    // Example 1: Ownership transfer
    let s1 = String::from("Hello");
    let s2 = s1; // Ownership of the String moves from s1 to s2
    println!("s2: {}", s2);
    // println!("s1: {}", s1); // Error: s1 no longer owns the value

    // Example 2: Cloning to retain ownership
    let s3 = String::from("World");
    let s4 = s3.clone(); // Creates a deep copy of the value
    println!("s3: {}, s4: {}", s3, s4);

    // Example 3: Borrowing (Immutable)
    let s5 = String::from("Rust");
    let len = calculate_length(&s5); // Borrow s5 without transferring ownership
    println!("The length of '{}' is {}", s5, len);

    // Example 4: Borrowing (Mutable)
    let mut s6 = String::from("Ownership");
    append_text(&mut s6); // Borrow s6 mutably to modify it
    println!("Modified string: {}", s6);

    // Example 5: Ownership with functions
    let s7 = String::from("Function");
    takes_ownership(s7); // Ownership of s7 is moved to the function
    // println!("s7: {}", s7); // Error: s7 is no longer valid
}

pub fn calculate_length(s: &String) -> usize {
    s.len() // Borrowing allows us to read the value without taking ownership
}

pub fn append_text(s: &mut String) {
    s.push_str(" Rules!"); // Mutable borrowing allows modification
}

pub fn takes_ownership(s: String) {
    println!("Takes ownership of: {}", s);
    // s is dropped here when the function ends
}

#[test]
fn data_copy() {
    let a = 10;
    let b = a; // ini tidak memindahkan kepemeilikan, tapi mengcopy value
    // jadi walau a diakses tidak akan error
    println!("a: {}, b: {}", a, b);
}

#[test]
fn ownership_movement() {
    let name1: String = String::from("Value name 1");
    println!("name1: {}", name1);

    let name2: String = name1; // ownership dari name1 pindah ke name2
    println!("name2: {}", name2);
    // println!("name1: {}", name1); // borrow of moved value: `name1` value borrowed here after move
}
#[test]
fn clone() {
    let name1: String = String::from("Value name 1");
    let name2 = name1.clone();
    println!("name1: {}", name1); // tetap bisa diakses
    println!("name2: {}", name2);
}

// // Fungsi untuk mengembalikan nama
// pub fn get_name() -> String {
//     String::from("Rust")
// }

// // Fungsi yang meminjam nama (borrow)
// pub fn print_name(name: &String) {
//     println!("Name is: {}", name);
// }

// #[test]
// fn test_memory_management() {
//     let my_name = get_name(); // Ownership berpindah ke my_name
//     print_name(&my_name); // Borrow my_name sebagai referensi
//     // my_name masih dapat digunakan di sini karena tidak diambil sendiri
//     println!("My name again is: {}", my_name);
// }
// ###########

/* If Expression
- Sama seperti bahasa pemrograman yang lain, Rust juga mendukung If Expression
- If expression digunakan untuk membuat percabangan kode sesuai dengan kondisi. Jika kondisi terpenuhi, maka blok kode If akan dieksekusi, jika kondisi tidak terpenuhi, klok kode If tidak akan dieksekusi

# Else Expression

- Ketika kondisi If tidak terpenuhi, kadang kita ingin melakukan sesuatu
- Kita bisa lakukan itu dengan Else Expression
- Blok di else akan dieksekusi jika kondisi If tidak terpenuhi

# Else If Expression

- Saat membuat If expression, kadang kita ingin membuat beberapa kondisi
- Untuk membuat beberapa kondisi, kita bisa gabungkan dengan Else If expression

let value = 9;
if value >= 8{
    println!("Value is greater than or equal to 8");
} else if value == 7 {
    println!("Value is 7");
} else {
    println!("Value is less than 7");
}


# Let Statement

- If di Rust adalah sebuah expression, artinya bisa menghasilkan value dan bisa digunakan dengan Let statement untuk mengisi data di variable
- Ini sangat berguna sehingga kita tidak perlu memasukkan nilai ke variable terpisah dengan deklarasi variable nya
let value = 9;
let result = if value >= 8 {
    "Value is greater than or equal to 8"
} else {
 // return "value"; // bisa ini
    "Value is less than 8" // atau ini juga bisa untuk return
};

// btw di rust cara return value ada 2 syntax
"valuenya" // tanpa titik koma ;
return "valuenya"; // dengan kata kunci return

*/
#[test]
fn if_expression() {
    let value = 6;
    if value >= 8 {
        println!("Value is greater than or equal to 8");
    } else if value == 7 {
        println!("Value is 7");
    } else {
        println!("Value is less than 7");
    }
}

#[test]
fn if_as_statement() {
    let value = 9;
    let reust: &str = if value >= 8 {
        "Value is greater than or equal to 8"
    } else if value == 7 {
        "Value is 7"
    } else {
        "Value is less than 7"
    };

    println!("result: {}", reust);
}

/* Loop

- Setiap bahasa pemrograman biasanya memiliki fitur untuk melakukan perulangan
- Rust mendukung beberapa cara untuk melakukan perulangan, pertama kita akan bahas tentang Loop
- Loop merupakan perintah di Rust digunakan untuk melakukan perulangan terus-menerus, sampai kita memerintahkannya untuk berhenti
- Jika kita tidak memerintahkan untuk berhenti, maka Loop tidak akan pernah berhenti melakukan perulangan

# Break dan Continue

- Untuk menghentikan perulangan, kita bisa menggunakan perintah break
- Selain break, ada juga perintah continue, yang artinya menghentikan perulangan saat ini, dan langsung dilanjutkan ke perulangan berikutnya


# Return Value di Loop

- Sama seperti If Expression, di Loop juga kita bisa mengembalikan nilai, sehingga bisa disimpan dalam variable dengan Let Expression
- Caranya kita bisa gunakan break lalu diikuti dengan nilai yang akan dikembalikan di Loop
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter > 10 {
        break counter * 2; // mengembalikan nilai counter * 2
    }
};

# Loop Label

- Kadang kita sering membuat Loop didalam Loop, dan ketika ingin menghentikan Loop paling atas dari Loop yang ada di dalam, maka hal itu tidak bisa dilakukan
- Loop memiliki fitur Label, dimana kita bisa memberi nama pada Loop
- Keuntungannya memberi Label pada loop adalah, kita bisa menghentikan Loop yang ingin kita hentikan dengan cara menyebutkan nama Label nya
let mut counter = 0;
'outer_loop: loop {
    println!("Outer loop iteration: {}", counter);
    let mut inner_counter = 0;

    'inner_loop: loop {
        println!("  Inner loop iteration: {}", inner_counter);
        inner_counter += 1;

        if inner_counter >= 3 {
            break 'inner_loop; // Hentikan inner loop
        }
    }

    counter += 1;
    if counter >= 2 {
        break 'outer_loop; // Hentikan outer loop
    }
}


*/
#[test]
fn loop_expression() {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter > 10 {
            break;
        } else if counter % 2 == 0 {
            continue;
        }
        println!("Counter value: {}", counter);
    }

    println!("Final counter value: {}", counter);
}

#[test]
fn loop_return_value() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter * 2; // mengembalikan nilai counter * 2
        }
    };
    println!("Result from loop: {}", result);
}
#[test]
fn loop_with_label() {
    let mut number = 1;
    'outer_loop: loop {
        println!("Outer loop iteration: {}", number);
        let mut inner_number = 1;

        'inner_loop: loop {
            println!("  Inner loop iteration: {}", inner_number);
            inner_number += 1;

            if inner_number > 3 {
                break 'inner_loop; // Hentikan inner loop
            }
        }

        number += 1;
        if number > 2 {
            break 'outer_loop; // Hentikan outer loop
        }
    }
}

/*

# While Loop
- `while` adalah salah satu cara untuk melakukan perulangan di Rust.
- Perulangan dengan `while` akan terus berjalan selama kondisi yang diberikan bernilai `true`.
- Ketika kondisi bernilai `false`, maka perulangan akan berhenti.
While Loop adalah jenis perulangan dimana memiliki kondisi
Jika kondisi masih terpenuhi, maka perulangan akan dilanjutkan
Namun jika perulangan tidak terpenuhi, maka perulangan akan dihentikan
While Loop mirip seperti Loop, bisa dihentikan menggunakan break dan continue

## Contoh Penggunaan
```rust
let mut counter = 0;
while counter < 5 {
    println!("Counter: {}", counter);
    counter += 1;
}
```
## Penjelasan
- Pada contoh di atas, perulangan akan terus berjalan selama nilai `counter` kurang dari 5.
- Setiap iterasi, nilai `counter` akan bertambah 1.
- Ketika `counter` mencapai 5, kondisi `counter < 5` akan bernilai `false`, dan perulangan berhenti.
## Test Function
```rust
#[test]
fn test_while_loop() {
    let mut counter = 0;
    while counter < 3 {
        println!("Counter: {}", counter);
        counter += 1;
    }
    assert_eq!(counter, 3); // Memastikan bahwa nilai counter adalah 3 setelah perulangan selesai
}
```
*/
#[test]
fn while_loop() {
    let mut counter = 0;
    while counter <= 20 {
        if counter % 2 == 0 {
            println!("Counter : {}", counter);
        }
        counter += 1;
        // di while loop juga bisa pakai break dan continue
    }
}

/* For loop

# Iterasi Array

- Salah satu yang biasa kita lakukan ketika menggunakan Array adalah, melakukan pengambilan semua data di Array dari data pertama sampai data terakhir
- Biasanya, kita akan menggunakan While Loop, lalu membuat variable untuk mengakses index nya
let array: [&str; 5] = ["A", "B", "C", "D", "E"];
// kalau pakai while loop seperti ini
let mut index = 0;
while index < array.len() {
    println!("Array element at index {}: {}", index, array[index]);
    index += 1;
}


# For Loop

- Rust menyediakan cara yang lebih mudah untuk melakukan pengambilan data dari Array menggunakan For Loop
let array: [&str; 5] = ["A", "B", "C", "D", "E"];
for value in array {
    println!("Value {}",value)
}


# Range

- Rust memiliki tipe data bernama Range
- Range adalah jarak antara start dan end
- Range merupakan tipe data Collection seperti Array, sehingga bisa dilakukan pengulangan menggunakan For Loop
- Data range akan dimulai dari start dan diakhiri sebelum end (exclusive)
- https://doc.rust-lang.org/std/ops/struct.Range.html

let range = 0..5; // dimulai dari 0 sampai sebelum 5
println!("Range: {:?}", range);
println!("Range start: {}", range.start);
println!("Range end: {}", range.end);
for i in range{
    println!("value i: {}", i);
}


# Range Inclusive

- Selain Range yang exclusive, Rust juga memiliki tipe data Range Inclusive
- Implementasinya berbeda dengan Range sebelumnya
- https://doc.rust-lang.org/std/ops/struct.RangeInclusive.html

let range_inclusive = 0..=5; // dimulai dari 0 sampai 5
println!("Range Inclusive: {:?}", range_inclusive);
println!("Start: {}", range_inclusive.start());
println!("End: {}", range_inclusive.end());
for i in range_inclusive{
    println!("value {}", array[i]);
}



*/

#[test]
fn array_iteration() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
    // kalau pakai while loop seperti ini
    let mut index = 0;
    while index < array.len() {
        println!("Array element at index {}: {}", index, array[index]);
        index += 1;
    }

    // ini dipermudah dengan for loop
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
    for value in array {
        println!("Value {}", value)
    }
}

#[test]
fn range() {
    let range = 0..5; // dari 0 sampai sebelum 5
    println!("Start {}", range.start); // 0
    println!("End {}", range.end); // 5
    // for i in range {
    //     println!("Value i: {}", i); // 1,... 4;  5 tidak ter print
    // }

    // implementasi dengan array
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
    // for i in range { // bisa seperti ini
    // bisa juga seperti ini
    for i in 0..5 {
        println!("array ke {} = {}", i, array[i]);
    }
}

#[test]
fn range_inclusive() {
    // bedanya adalah range inclusive ini termasuk nilai akhirnya
    let range_inclusive = 0..=4; // dimulai dari 0 sampai 4
    println!("Range Inclusive: {:?}", range_inclusive);
    println!("Start: {}", range_inclusive.start());
    println!("End: {}", range_inclusive.end());

    // implementasi dengan array
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
    for i in range_inclusive {
        println!("value {}", array[i]);
    }
}

/* Function
- function di rust adalah kumpulan kode yang memiliki nama, dan kegunannya adalah supaya bisa dipanggil
- ditandai dengan kata kunci fn
- function main, yaitu function yang dipanggil oleh rust pertama kali saat program dijalankan
- di rust menamai function biasa dilakukan dengan _ (snake_case) misal: fn get_name()
- dan function bisa memiliki parameter

# Parameter
- parameter adalah data yang dikirim ke function saat function dipanggil
- ketika function yang memiliki parameter dipanggil, maka kita wajib memberi value pada parameter tersebut
- beberapa programmer memanggil parameter dengan sebutan `argument`
- parameter di function bisa satu atau lebih, dan setiap parameter bisa menggunakan tipe data apapun dan wajib di deklarasikan tipe datanya diawal
fn greet(name: &str) {
    println!("Hello, {}", name);
}

# Return value
- nilai atau hasil yang dikembalikan oleh function disebut return value
- jika sebuah function ingin mengembalikan value kita bisa mendeklarasikan dengan `->` lalu diikuti dengan tipe data value-nya
- di rust baris terakhir di function adalah nilai yang akan dikembalikan, kecuali ada kata kunci return
fn get_name() -> String {
    String::from("Rust") // bisa seperti ini
    // return String::from("Rust"); // atau bisa juga seperti ini
}

# Recursive function
- function yang memanggil dirinya sendiri disebut recursive function
- rust memperbolehkan sebuah function memanggil dirinya sendiri
- namun perlu diperhatikan, recursive function harus memiliki kondisi berhenti, supaya tidak terjadi infinite loop
*/
fn say_hello() {
    println!("Hello say");
}

fn say_goodbye(first_name: &str, last_name: &str) {
    println!("Goodbye, {} {}", first_name, last_name);
}

#[test]
fn test_function() {
    say_hello();

    say_goodbye("Awalun", "Akhirun");
    say_goodbye("Dennis", "Boone");
}

fn factorial_loop(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }

    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }

    result // tanpa ; 
    // return result; // dengan ;
}
#[test]
fn test_factorial_loop() {
    let result: i32 = factorial_loop(5);
    println!("Factorial 5 adalah {}", result);

    let result2: i32 = factorial_loop(-10);
    println!("Factorial -10 adalah {}", result2); // ini harusnya 0
}

fn print_text(value: String, times: u32) {
    if times == 0 {
        return;
    } else {
        println!("{}", value);
    }

    print_text(value, times - 1);
}
#[test]
fn test_print_text() {
    print_text(String::from("Oke"), 10);
}

fn factorial_recursive(n: u32) -> u32 {
    println!("n ke {n}");
    if n <= 1 {
        println!("n sudah 1 waktunya return");
        return 1;
    }

    n * factorial_recursive(n - 1)
}
#[test]
fn test_factorial_recursive() {
    let result = factorial_recursive(5);
    println!("Factorial recursive 5 adalah {}", result);
}

/* Ownership di function
- Tipe data yang disimpan di Heap, ketika kita kirim sebagai parameter difunction, maka secara otomatis ownership-nya pindah ke function tersebut
- Setelah function selesai dieksekusi, maka variable parameter tersebut akan di drop, sehingga data di Heap akan dihapus dan variable tersebut tidak bisa diakses lagi
- Namun untuk tipe data yang disimpan di Stack, ketika dikirim ke function sebagai parameter, maka data tersebut akan di copy, sehingga variable asli tetap bisa diakses
fn main() {
    let name = String::from("Rust"); // tipe String disimpan di heap
    greet(name); // ownership dari name pindah ke greet function
    println!("name: {}", name); // ini akan error karena name sudah tidak valid lagi

    let age: i32 = 20; // tipe i32 disimpan di stack
    print_age(age); // data age di copy ke print_age function
    println!("age: {}", age); // ini tetap bisa diakses
}

# Return value ownership
- Value Heap yang kita kembalikan di function, secara otomatis ownership-nya pindah ke variable / apapun yang memanggil function tersebut
- Sedangkan jika Value Stack, maka return value hanya akan meng-copy oleh variable / apapun yang memanggil function tersebut
fn get_full_name(first_name: String, last_name: String) -> String {
    let full_name = format!("{} {}", first_name, last_name);
    full_name // ownership dari full_name pindah ke pemanggil function
}

# Mengembalikan Ownership
- Pada kasus tertentu, kita mungkin ingin mengembalikan ownership dari parameter ke pemanggil function
- hal itu bisa dilakukan dengan return value ber tipe `tuple` dan kita perlu assign kembali ke variable pemiliknya
fn some_function(data: String) -> (String, String) {
    let processed_data = format!("Processed: {}", data);
    (data, processed_data) // mengembalikan ownership dari data dan processed_data
}

# Problem dengan return value ownership
- jika kita tidak ingin mengambil ownerhsip dari parameter, maka jika tiap membuat function kita harus membuat return value tuple, dan lama-lama ini akan menyulitkan
- bahkan akan sulit dibaca dan dimengerti function-nya
- untung-nya rust ada solusi untuk masalah ini, namanya adalah Reference dan Borrowing
*/
fn print_number(number: i32) {
    println!("Number: {}", number);
}
fn print_string(text: String) {
    println!("Text: {}", text);
}

#[test]
fn test_ownership_fn() {
    let number: i32 = 42; // tipe i32 disimpan di stack
    print_number(number); // data di copy
    println!("number asli: {}", number); // tetap aman

    let name = String::from("Rust"); // tipe String disimpan di heap
    print_string(name); // ownership pindah ke function
    // println!("name asli: {}", name); // ini akan error karena name sudah tidak valid lagi
    //borrow of moved value: `name` value borrowed here after move
}

fn get_full_name(first_name: String, last_name: String) -> String {
    format!("{} {}", first_name, last_name) // ownership dari full_name pindah ke pemanggil function
}
fn get_sum(a: i32, b: i32) -> i32 {
    a + b // ini hanya meng-copy value
}
#[test]
fn test_ownership_fn_return_value() {
    let first_name = String::from("Grace");
    let last_name = String::from("Coleman");
    let full_name = get_full_name(first_name, last_name); // ownership pindah ke full_name
    println!("Full name: {}", full_name);
    // println!("First name: {}", first_name); // error karena first_name sudah tidak valid lagi (sudah di drop)
    // println!("Last name: {}", last_name); // ini juga error

    let number1 = 90;
    let number2: i32 = 10;
    let sum12 = get_sum(number1, number2); // ini copy dari number1 dan number2
    println!("Sum: {}", sum12);
    println!("Number1: {}", number1); // tetap bisa diakses
    println!("Number2: {}", number2); // tetap bisa diakses
}

fn get_full_name_with_ownership(first_name: String, last_name: String) -> (String, String, String) {
    let full_name = format!("{} {}", first_name, last_name);
    (first_name, last_name, full_name) // mengembalikan ownership dari first_name, last_name, dan full_name
}
#[test]
fn test_ownership_fn_with_ownership() {
    let first_name = String::from("Juan");
    let last_name = String::from("Barber");
    // let (_,_,full_name) = get_full_name_wit h_ownership(first_name, last_name); // bisa seperti ini jika tidak terpakai
    // println!("first_name {} and last_name {}",first_name,last_name); // ini error
    let (first_name, last_name, full_name) = get_full_name_with_ownership(first_name, last_name);

    println!("first_name {} and last_name {}", first_name, last_name); // ini tidak error karena di assign ulang
    println!("fullname is {}", full_name);
}

/* References
- reference adalah pointer ke data asli di Heap, jadi bukan copy atau pindah ownership, datanya sendiri dimiliki oleh variable lain, bukan si reference
- Refrence akan dijamin menunjuk value yang valid selama alur hidup reference tersebut, jika sudah selesai maka reference akan dihapus tapi tidak dengan data yang ditunjuknya
- Reference ditandai dengan simbol & sebelum tipe data, dan kita bisa buat banyak reference ke data yang sama dalam satu waktu
- sebenarnya kita sudah menggunakan reference pada tipe data str yang kita pakai dengan nama &str, hal ini karena default-nya adalah reference ke str

# Borrowing
- Aksi reference juga biasa disebut dengan istilah Borrowing
- kalau diibaratkan, reference itu seperti meminjam data dari orang lain, kita hanya meminjamnya, bukan memiliki data tersebut dan kita harus mengembalikan ke owner (pemilik)
- saat kita mencoba memodifikasi value dari reference, maka secara default hal itu tidak bisa dilakukan karena secara default reference adalah immutable, walaupun variable owner dari reference-nya mutable
fn change_value(value: &String){
    value.push_str(" New Value"); // ini akan error karena reference bersifat immutable
}

# Mutable Reference
- Pada kasus dimana kita perlu mengubah value dari reference, maka kita bisa menggunakan Mutable Reference
- Caranya adalah dengan tanda &mut sebelum tipe data, contoh: &mut String
- Namun perlu diperhatikan, pada satu waktu hanya boleh ada satu mutable reference ke data yang sama
- Dan owner dari data tersebut juga harus mutable, kalau ownernya immutable maka kita tidak bisa mengubah data owner dari reference-nya

# Dangling Pointer
- Dangling Pointer adalah kondisi dimana sebuah reference menunjuk ke data yang sudah tidak valid lagi atau sudah tidak ada dimemory
- Di Rust, hal ini tidak diperbolehkan, contoh ketika kita ingin mengembalikan reference dalam function, maka secara otomatis value akan dihapus dari scope function
- pada kasus seperti ini, Rust akan menganggap hal ini error, karena berpotensi terjadi dangling pointer
- biasanya programmer golang sering kali membuat function yang mengembalikan pointer
fn get_value(first_name:&String,last_name: &String) -> &String { // ini akan error
    let full_name = format!("{} {}", first_name, last_name);
    &full_name // ini akan error karena full_name akan di drop saat function selesai

# Solusi Dangling Pointer
- jika memang data yang dikembalikan dibuat didalam function, maka kita harus mengebalikan dalam bentuk value langsung, bukan reference
- atau kita bisa mengeluarkan variable owner dari value diluar function, agar masuk variable scope, sehingga rust tidak menghapus variable dan value tersebut setelah function selesai di eksekusi
// cara pertama lebih baik
*/

fn get_full_name_with_reference(first_name: &String, last_name: &String) -> String {
    format!("{} {}", first_name, last_name)
}
#[test]
fn test_fn_with_reference() {
    let first_name = String::from("Howard");
    let last_name = String::from("Daniel");

    // pakai & sebelum variable untuk mengirim reference
    let full_name: String = get_full_name_with_reference(&first_name, &last_name); // mengirim reference, dan tanpa memindahkan ownership
    println!("full_name: {}", full_name);
    println!("first_name: {}", first_name); // tidak error
    println!("last_name: {}", last_name); // tidak error
}

// fn change_value(value: &String) {
//     // value.push_str(" New Value"); // ini akan error karena reference bersifat immutable
// }
fn change_value(value: &mut String) {
    value.push_str(" Random Word");
}
#[test]
fn test_change_value() {
    let mut value: String = String::from("Original Value");
    // change_value(&value);
    change_value(&mut value);
    change_value(&mut value); // ini bisa dilakuakn karena `&mut value` diatas berbeda dengan ini

    println!("value: {}", value);

    // ini masih bisa dilakukan
    let valueBorrow1 = &mut value;
    // let valueBorrow2 = &mut value; // ini tidak boleh karena mutable reference hanya boleh satu saja pada satu waktu dan itu masih dipakai oleh valueBorrow1
    // cannot borrow `value` as mutable more than once at a time
    // let valueBorrow3 = &value; // ini juga tidak boleh jika ada mutable reference maka immutable reference ke owner tersebut tidak boleh ada dalam satu waktu, walaupun ini reference biasa dan bukan mutable reference

    change_value(valueBorrow1);

    println!("value after valueBorrow1: {}", value);
    // let valueBorrow4 = &value; // ini baru boleh karena valueBorrow1 sudah tidak dipakai lagi
    // let valueBorrow5 = &value; // dan boleh sebanyak-banyak nya kalau immutable reference / reference biasa
}

// missing lifetime specifier
// this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `first_name` or `last_name`
// fn get_value(first_name:&String,last_name: &String) -> &String { // ini akan error
//     let full_name = format!("{} {}", first_name, last_name);
//     &full_name // ini akan error karena full_name akan di drop saat function selesai
// }

fn get_full_name_ding(first_name: &String, last_name: &String) -> String {
    let full_name = format!("{} {}", first_name, last_name);
    full_name // mengembalikan value langsung, bukan reference
}
#[test]
fn test_get_full_name_ding() {
    let first_name = String::from("Howard");
    let last_name = String::from("Daniel");

    let full_name: String = get_full_name_ding(&first_name, &last_name); // mereturn ownership baru langsung
    println!("full_name: {}", full_name);
    println!("first_name: {}", first_name); // tidak error
    println!("last_name: {}", last_name); // tidak error
}

/* Slice
- Slice adalah reference ke sebagian elemen dari data collection (misal array)
- Karena slice adalah reference, jadi dia tidak punya ownership
- Contoh misal kita punya array dengan total data 10, kita mau ambil 5 data terdepan, maka kita bisa membuat Slice sebagai reference data dari data ke-1 sampai ke-5

# Range
- Saat kita ingin mengambil sebagian data Collection, kita butuh menentukan range untuk Slice yang akan kita ambil
- Rust sendiri memiliki banyak jenis range, sebelumnya kita sudah bahas tentang Range (exclusive) dan Range Inclusive, selain itu masih ada yang lain
- https://doc.rust-lang.org/std/ops/index.html#structs

*/
#[test]
fn slice_reference() {
    let array: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let slice1: &[i32] = &array[..]; // reference data seluruh index array, tidak memindahkan ownership dan juga tidak mengcopy data
    println!("slice1: {:?}", slice1);

    let slice2: &[i32] = &array[0..5]; // reference data dari index ke 0 sampai sebelum 5, tidak memindahkan ownership dan juga tidak mengcopy data
    println!("slice2: {:?}", slice2);

    let slice3: &[i32] = &array[5..]; // reference data dari index ke 5 sampai akhir, tidak memindahkan ownership dan juga tidak mengcopy data
    println!("slice3: {:?}", slice3);

    // let slice4 = &&&&&slice3; // tidak error
    let slice4 = slice3; // tidak error dan ingat ini bukan mengcopy data, tapi hanya membuat reference baru ke data yang sama
    println!("slice4: {:?}", slice4);
}

/* String Slice
- sebelumnya kita sudah menggunakan String Slice, yaitu &str, hal ini sebenarnya berarti &str itu adalah reference ke data string
- saat kita menggunakan tipe data String, kita juga mengambil sebagian karakter di String, hasil dari sebagian data itu adalah &str (String Slice)
- karena &str adalah reference, maka sebenarnya dia tidak memiliki ownership, oleh karena itu ketika kita assign ke variable lain atau ke function, yang dicopy sebenarnya adalah reference-nya, datanya tetap menggunakan data yang sama

*/
#[test]
fn string_slice() {
    let full_name: String = String::from("Roy Francis");

    let first_name: &str = &full_name[0..3];
    println!("first_name {}", first_name);

    let last_name: &str = &full_name[4..];
    println!("last_name {}", last_name);
}

/* Struct
- Struct adalah tipe data mirip Tuple yang bisa digunakan untuk menampung beberapa data dengan tipe yang berbeda
- Yang membedakan dengan Tuple, pada Struct, kita bisa memberi nama untuk tiap data-nya, atau dibilang field, sehingga lebih jelas dibanding Tuple yang hanya menggunakan number
- Dengan menambahkan nama pada data di Struct, secara otomatis kita tidak perlu harus menentukan urutan posisi data yang selalu sama, bisa berubah-ubah seiring pembuatan kode
- Untuk membuat Struct, kita bisa gunakan kata kunci `struct`
- Panduan Rust, nama struct harus diawali huruf Besar dan CamelCase, walaupun bisa pakai huruf kecil dan snake_case, tapi nanti akan ada warning
struct Person {
    first_name: String,
    last_name: String,
    //...
}

# Membuat Instance dari Struct
- Jadi Struct itu sebelum digunakan kita harus buat dahulu definisi dari Struct nya
- Setelah kita membuat definisi Struct-nya, selanjutnya kita bisa membuat instance / value / "Object" nya dari Struct yang sudah kita buat
- Saat membuat instance dari Struct, kita wajib menentukan semua value untuk field dari Struct-nya

# Menggunakan Struct di Function
- Struct sama seperti tipe data lainnya, kita bisa gunakan dimanapun
- Kita bisa gunakan Struct sebagai parameter di function, atau return value di function

# Init Shorthand
- Kadang ada kasus kita ingin membuat value untuk field di Struct dari variable yang sudah ada
- Jika misalnya nama variable sama dengan nama field, kita tidak perlu sebutkan nama field secara eksplisit
- Fitur ini bernama Init Shorthand
- tapi ingat ownership-nya akan berpindah
```rust
let first_name = String::from("Hello")
let person:Person = Person {
    first_name, // dari sini variable first_name sudah tidak valid
    last_name:String::from("last name")
}
```


# Struct Update Syntax
- Sama seperti tipe data lainnya, saat kita buat instance Struct sebagai immutable, maka semua field di instance tersebut tidak bisa diubah
- Jika kita ingin mengubahnya, kita harus buat instance Struct dalam bentuk mutable variable
- Struct memiliki fitur bernama Struct Update Syntax, ini digunakan untuk membuat instance baru dari instance yang sudah ada
- Bahkan, kita bisa membuat instance baru sekaligus mengubah beberapa field yang kita mau
let person2: Person = Person { ..person}

# Masalah dengan Struct Update Syntax
- Namun saat menggunakan struct update syntax, hati-hati dengan field yang memiliki value di Heap, karena ownershipnya secara otomatis akan dipindahkan ke field di instance baru
- Oleh karena itu, secara otomatis instance lama tidak bisa digunakan karena value di field nya sudah dipindahkan ownershipnya ke instance baru
- Atau kita bisa melakukan clone data field nya, jika memang tidak mau memindahkan ownershipnya

# Tuple Struct
- Seperti di awal dijelaskan, bahwa Struct mirip seperti Tuple
- Seandainya kita ingin membuat Struct seperti Tuple, kita juga bisa buat Struct tanpa menyebutkan nama field nya
- Namun ketika kita buat Struct jenis ini, maka cara mengakses field nya sama seperti ketika kita membuat Tuple
- Ini cocok ketika kita kita mau membuat Tuple dengan data banyak, agar lebih sederhana, dibuat dalam bentuk Struct
struct GeoPoint(f64, f64);

# Struct tanpa Field
- Field di Struct tidak wajib, artinya jika kita buat Struct tanpa field sama sekali, hal itu diperbolehkan
- Struct tanpa Field itu sama saja dengan tipe data Unit ()
- Apa gunanya Struct tanpa Field? Sekarang mungkin tidak terlalu terlihat gunanya, tapi nanti setelah belajar Trait, kita mungkin akan sering membuat Struct tanpa field. Ini akan kita bahas di materi Trait
struct Nothing;

# Reference Field di Struct
- Sebelumnya kita menggunakan tipe data String yang disimpan di Heap, bagaimana jika kita menggunakan tipe data &str (String Slice) yang merupakan tipe data reference?
- Struct Field bisa bertipe data reference, namun untuk melakukan itu kita harus menggunakan Lifetime, dan untuk ini kita akan bahas di materi Lifetime
*/
struct Person {
    first_name: String,
    // last_name: &str, //missing lifetime specifier, expected named lifetime parameter    // gak bisa
    last_name: String,
    age: u8,
}

fn print_person(person: &Person) {
    println!("person.first_name = {}", person.first_name);
    println!("person.last_name = {}", person.last_name);
    println!("person.age = {}", person.age);
}

#[test]
fn struct_person() {
    let person: Person = Person {
        first_name: "Jean".to_string(),
        last_name: String::from("Shelton"),
        age: 48,
    };

    print_person(&person);
}

#[test]
fn test_struct_person() {
    let first_name = String::from("Gilbert");

    let person: Person = Person {
        first_name, // dari sini variable first_name sudah tidak valid dan tidak bisa diakses
        last_name: String::from("Lee"),
        age: 4,
    };
    // println!("first_name: {}", first_name); // ini akan error // borrow of moved value: `first_name`

    // person.age = 1; // ini akan error, karena person bukan mutable, perlu dibuat `let mut person = ...` supaya bisa diubah
    print_person(&person);

    let mut mutable_person: Person = Person {
        first_name: String::from("Daisy"),
        last_name: String::from("Ward"),
        age: 68,
    };
    mutable_person.age = 20; // ini bisa diubah

    print_person(&mutable_person);

    let person2: Person = Person {
        age: 10,
        // first_name: person.first_name.clone(), // kalau pakai clone, maka ownership dari person.first_name tetap ada di person,
        ..person // ini juga memindahkan ownership tipe data yang disimpan di Heap seperti first_name, dan last_name karena pakai String
                 // ..person, age:10 // ini tidak di-allow tidak boleh ada , (koma)
    };
    print_person(&person2);

    // println!("person.first_name : {}", person.first_name); // tidak bisa karena ownership-nya pindah ke person2.first_name
    //borrow of moved value: `person.first_name` // move occurs because `person.first_name` has type `String`, which does not implement the `Copy` trait
}

struct GeoPoint(f64, f64);
#[test]
fn tuple_struct() {
    let geo_point: GeoPoint = GeoPoint(-6.123, 100.123);
    println!("geo_point.0 {}", geo_point.0);
    println!("geo_point.1 {}", geo_point.1);
}

struct Nothing;
#[test]
fn test_nothing() {
    let _nothing1: Nothing = Nothing;
    let _nothing2: Nothing = Nothing {};
}

/* Method
- Method sebenarnya sama seperti function, membuatnya menggunakan fn, punya nama, bisa punya parameter dan bisa punya return value
- Yang membedakan dengan function adalah, method itu tidak berdiri sendiri, melainkan menempel di Struct, Enum atau Trait. Enum dan Trait akan dibahas di materi terpisah
- Pada method, parameter pertama selalu menggunakan self
- self adalah representasi dari instance dari Struct dimana method tersebut dipanggil

# Membuat Method
- Untuk membuat method, kita harus tentukan ingin meletakkan di Struct mana, caranya menggunakan kata kunci impl, lalu diikuti dengan nama Struct nya
- Lalu didalamnya, kita bisa lakukan seperti kita membuat function
- Untuk mengakses semua field yang ada di instance Struct, kita bisa gunakan parameter self pertama di Method
- Biasanya parameter self dibuat dalam bentuk reference, agar ownership nya tidak diambil oleh Method yang dipanggil tersebut

# Associated Functions
- Setiap function yang dibuat dalam impl kita sebut dengan Associated Functions, karena terkait dengan tipe data yang kita tentukan di impl
- Associated Functions yang memiliki parameter self artinya adalah Method, dan dipanggil setelah kita membuat instance nya
- Namun, kita juga bisa membuat function tanpa parameter self, yang artinya function tersebut tidak terhubung dengan instance-nya
- Untuk memanggil Associated Functions yang bukan Method, kita bisa langsung gunakan NamaType.nama_function()
- Biasanya Associated Functions bukan Method, digunakan untuk membuat instance dari Type nya
```rust
impl GeoPoint {
    fn new(long: f64, lat: f64) -> GeoPoint { // ini disebut associated function bukan method
        GeoPoint (long, lat)
    }
}
#[test]
fn test_method_new() {
    let geo_point: GeoPoint = GeoPoint::new(-6.200000, 106.816666); // cara akses-nya pakai ::
    println! ("long: {}", geo_point.0);
    println! ("lat: {}", geo_point.1);
}

```

*/
impl Person {
    fn say_hello(&self, name: &str) {
        // dan jangan lupa untuk pakai &self, supaya ownership tidak berpindah, kalau pakai self saja akan berpindah dan menyebabkan error
        println!("Hello {}, my name is {}", name, self.first_name);
    }
}

#[test]
fn test_method() {
    let person: Person = Person {
        first_name: String::from("Daisy"),
        last_name: String::from("Ward"),
        age: 68,
    };

    person.say_hello("Paul");
    // println!("{}", person.first_name)
}

impl GeoPoint {
    // ini disebut associated function bukan method
    fn new(long: f64, lat: f64) -> GeoPoint {
        GeoPoint(long, lat)
    }
}
#[test]
fn test_method_new() {
    let geo_point: GeoPoint = GeoPoint::new(-6.200000, 106.816666); // cara akses-nya pakai ::
    println!("long: {}", geo_point.0);
    println!("lat: {}", geo_point.1);

    // geo_point.new(-6.0,10.0) // ini akan error karena memang tidak bisa dipanggil
}
