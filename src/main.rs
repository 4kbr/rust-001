use std::thread;

fn main() {
    // secara default semua program di rust berjalan di thread, walaupun kita tidak menggunakan thread sama sekali
    // ini contoh cara kita get thread saat ini yang sedang digunakan\
    let current_thread = thread::current();
    println!(
        "Hello, world!, current thread = {}",
        current_thread.name().unwrap() //Hello, world!, current thread = main
    );
}

#[cfg(test)]
mod tests {
    use std::thread::{self, JoinHandle};
    use std::time::Duration;

    #[test]
    fn test_thread() {
        // kalau tidak pakai thread::spawn(...) , maka for loop ini tetap akan ditunggu dan tidak concurent ke syntax lain
        thread::spawn(|| {
            for i in 0..=5 {
                println!("Iteration: {}", i);
                thread::sleep(Duration::from_secs(1));
            }
        });
        // thread::spawn(|| {
        //     println!("This is a thread running!");
        // });
        println!("Application finish");
        thread::sleep(Duration::from_secs(7)); // Give the thread time to run
        println!("Application done");
    }

    #[test]
    fn test_join_thread_1() {
        let handle = thread::spawn(|| {
            println!("This is a thread with join!");
        });
        handle.join().unwrap(); // Wait for the thread to finish
    }

    // cargo test tests::test_join_thread_2 -- --nocapture
    #[test]
    fn test_join_thread_2() {
        let handle: JoinHandle<i32> = thread::spawn(|| {
            let mut counter = 0;
            for i in 1..=5 {
                println!("counter: {}", i);
                thread::sleep(Duration::from_secs(1));
                counter = counter + 1;
            }
            return counter;
        });
        // .join() ini membuat syntax menunggu sampai proses handle selesai, jadi hati-hati penggunaan-nya
        let result = handle.join();
        match result {
            Ok(counter) => println!("total counter: {}", counter),
            Err(error) => println!("error: {:?}", error),
        }
        println!("application finish");
    }

    fn calculate() -> i32 {
        let mut counter = 0;
        let current = thread::current();
        for i in 1..=5 {
            thread::sleep(Duration::from_secs(1));
            counter = counter + 1;

            match current.name() {
                Some(name) => println!("{} : counter: {}", name, i),
                None => println!("{:?} : counter: {}", current.id(), i),
            }
        }

        // println!("Calculate: {}", i);
        // thread::sleep(Duration::from_secs(1));

        counter
    }

    #[test]
    fn test_calculate_sequential() {
        // walau terlihat rapih, tapi metode ini akan memakan waktu lebih lama, karena menghitung 1 persatu
        calculate(); // hitung 1 sampai selesai
        calculate(); // baru panggil ini
        println!("Sequential test done");
    }

    #[test]
    fn test_calculate_with_thread() {
        // sementara ini menghitung keduanya berbarengan
        let handle1 = thread::spawn(|| calculate()); // hitung bareng
        let handle2 = thread::spawn(|| calculate()); // hitung bareng
        handle1.join().unwrap();
        handle2.join().unwrap();
        // hasil print
        // ThreadId(3) : counter: 1
        // ThreadId(4) : counter: 1
        // ThreadId(3) : counter: 2
        // ThreadId(4) : counter: 2
        // ThreadId(4) : counter: 3
        // ThreadId(3) : counter: 3
        println!("Threaded test done");
    }

    #[test]
    fn test_closure() {
        let current_thread = thread::current();
        println!("current thread = {}", current_thread.name().unwrap()); // current thread = tests::test_closure
        let name = String::from("Eko");
        let closure = move || {
            thread::sleep(Duration::from_secs(2));
            println!("Hello, {}", name);
        };

        // println!("name is {}", name); // ini error karena ownership sudah di move ke closure

        let handler = thread::spawn(closure);
        handler.join().unwrap();
    }

    #[test]
    fn test_move_keyword() {
        let data = String::from("Hello from move!");
        let handle = thread::spawn(move || {
            println!("{}", data); // Ownership of `data` is moved here
        });
        handle.join().unwrap();
    }

    #[test]
    fn test_current_thread() {
        let current_thread = thread::current();
        println!("Current thread name: {:?}", current_thread.name());
    }

    #[test]
    fn test_thread_factory() {
        let builder = thread::Builder::new().name("custom-thread".to_string());
        let handle = builder
            .spawn(|| {
                println!("This is a custom thread!");
            })
            .unwrap();
        handle.join().unwrap();
    }
}
