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
