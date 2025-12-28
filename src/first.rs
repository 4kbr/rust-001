// mod third; bukan disini tapi di src/main.rs
use crate::third::say_halo as say_halo_third;

// kalau beda file tidak perlu ada
// mod { .. lagi
pub fn say_halo() {
    println!("Hello from first module");

    say_halo_third();
}

// }

// mengakses super function / method
pub mod second {
    pub mod third {
        pub fn say_hello() {
            // crate::first::say_halo(); // ini juga bisa
            // super pertama ke luar mod third, super kedua ke luar mod second
            // super::super::say_halo();
            println!("say hello dari dalam mod second, third didalam first.rs");
            super::super::say_halo_third();
        }
    }
}
