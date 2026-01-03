fn main() {
    println!("Hello, world!");
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

    fn calculate() {
        for i in 1..=5 {
            println!("Calculate: {}", i);
            thread::sleep(Duration::from_secs(1));
        }
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
        println!("Threaded test done");
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
