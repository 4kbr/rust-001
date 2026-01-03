fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::thread;
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
    fn test_join_thread() {
        let handle = thread::spawn(|| {
            println!("This is a thread with join!");
        });
        handle.join().unwrap(); // Wait for the thread to finish
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
