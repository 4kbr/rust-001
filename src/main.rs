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
    use std::sync::mpsc;
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
        let factory = thread::Builder::new().name("My Thread".to_string());
        let handler = factory
            .spawn(calculate)
            .expect("Failed to create a new thread");
        let total = handler.join().unwrap();
        println!("total counter: {}", total);
    }
    #[test]
    fn test_thread_factory_gen() {
        let builder = thread::Builder::new().name("custom-thread".to_string());
        let handle = builder
            .spawn(|| {
                println!("This is a custom thread!");
            })
            .unwrap();
        handle.join().unwrap();
    }

    //  channel
    /*
    Fungsi ini mendemonstrasikan channel paling dasar:
    - Satu sender
    - Satu receiver
    - Satu data dikirim
    */
    fn basic_channel_example() {
        println!("=== basic_channel_example ===");

        let (sender, receiver) = mpsc::channel::<String>();

        thread::spawn(move || {
            let message = String::from("Halo dari thread!");
            println!("[Sender] Mengirim pesan: {}", message);
            sender.send(message).unwrap();
            println!("[Sender] Pesan sudah dikirim");
        });

        println!("[Receiver] Menunggu pesan...");
        let received = receiver.recv().unwrap();
        println!("[Receiver] Menerima pesan: {}", received);

        println!("=== selesai basic_channel_example ===\n");
    }

    #[test]
    fn test_basic_channel_example() {
        basic_channel_example();
    }

    /*
    Contoh paling dasar:
    - handler1 → sender
    - handler2 → receiver
    - hanya satu pesan
    */
    fn basic_channel_with_handler() {
        println!("=== basic_channel_with_handler ===");

        let (sender, receiver) = mpsc::channel::<String>();

        let handler1 = thread::spawn(move || {
            println!("[Sender / handler1] Thread dimulai");

            let msg = String::from("Halo dari handler1");
            println!("[Sender / handler1] Mengirim: {}", msg);
            sender.send(msg).unwrap();

            println!("[Sender / handler1] Selesai");
        });

        let handler2 = thread::spawn(move || {
            println!("[Receiver / handler2] Thread dimulai");

            println!("[Receiver / handler2] Menunggu pesan...");
            let received = receiver.recv().unwrap();
            println!("[Receiver / handler2] Menerima pesan: {}", received);

            println!("[Receiver / handler2] Selesai");
        });

        handler1.join().unwrap();
        handler2.join().unwrap();

        println!("=== selesai basic_channel_with_handler ===\n");
    }

    #[test]
    fn test_basic_channel_with_handler() {
        basic_channel_with_handler();
    }

    /*
    Fungsi ini menunjukkan:
    - Pengiriman banyak pesan
    - Delay agar terlihat prosesnya
    */
    fn multiple_messages_example() {
        println!("=== multiple_messages_example ===");

        let (sender, receiver) = mpsc::channel();

        thread::spawn(move || {
            let messages = vec!["Pesan pertama", "Pesan kedua", "Pesan ketiga"];

            for msg in messages {
                println!("[Sender] Mengirim: {}", msg);
                sender.send(String::from(msg)).unwrap();
                thread::sleep(Duration::from_secs(1));
            }

            println!("[Sender] Semua pesan dikirim");
        });

        for received in receiver {
            println!("[Receiver] Menerima: {}", received);
        }

        println!("=== selesai multiple_messages_example ===\n");
    }

    #[test]
    fn test_multiple_messages_example() {
        multiple_messages_example();
    }
    /*
    Contoh:
    - handler1 mengirim banyak pesan
    - handler2 menerima secara streaming
    */
    fn multiple_messages_with_handler() {
        println!("=== multiple_messages_with_handler ===");

        let (sender, receiver) = mpsc::channel();

        let handler1 = thread::spawn(move || {
            println!("[Sender / handler1] Thread dimulai");

            let messages = vec!["Pesan pertama", "Pesan kedua", "Pesan ketiga"];

            for msg in messages {
                println!("[Sender / handler1] Mengirim: {}", msg);
                sender.send(String::from(msg)).unwrap();
                thread::sleep(Duration::from_secs(1));
            }

            println!("[Sender / handler1] Semua pesan dikirim");
        });

        let handler2 = thread::spawn(move || {
            println!("[Receiver / handler2] Thread dimulai");

            for received in receiver {
                println!("[Receiver / handler2] Menerima: {}", received);
            }

            println!("[Receiver / handler2] Channel ditutup, receiver selesai");
        });

        handler1.join().unwrap();
        handler2.join().unwrap();

        println!("=== selesai multiple_messages_with_handler ===\n");
    }
    #[test]
    fn test_multiple_messages_with_handler() {
        multiple_messages_with_handler();
    }

    #[test]
    fn test_channle_queue() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();
        let handler1 = thread::spawn(move || {
            for i in 0..5 {
                thread::sleep(Duration::from_secs(2));
                sender.send("Hello from thread".to_string());
                sender.send("Exit".to_string());
            }
        });
        let handler2 = thread::spawn(move || {
            loop {
                let message = receiver.recv().unwrap();
                if message == "Exit" {
                    break;
                }
                println!("{}", message);
            }
        });

        let _ = handler1.join();
        let _ = handler2.join();
    }

    /*
    Fungsi ini menunjukkan:
    - Banyak sender (clone sender)
    - Satu receiver
    */
    fn multiple_senders_example() {
        println!("=== multiple_senders_example ===");

        let (sender, receiver) = mpsc::channel();

        let sender2 = sender.clone();

        thread::spawn(move || {
            println!("[Sender 1] Mengirim pesan");
            sender.send(String::from("Pesan dari sender 1")).unwrap();
        });

        thread::spawn(move || {
            println!("[Sender 2] Mengirim pesan");
            sender2.send(String::from("Pesan dari sender 2")).unwrap();
        });

        for received in receiver {
            println!("[Receiver] Menerima: {}", received);
        }

        println!("=== selesai multiple_senders_example ===\n");
    }
    #[test]
    fn test_multiple_senders_example() {
        multiple_senders_example();
    }

    /*
    Contoh:
    - handler1 dan handler2 sebagai sender
    - handler3 sebagai receiver
    */
    fn multiple_senders_with_handler() {
        println!("=== multiple_senders_with_handler ===");

        let (sender, receiver) = mpsc::channel();

        let sender2 = sender.clone();

        let handler1 = thread::spawn(move || {
            println!("[Sender / handler1] Mengirim pesan");
            sender.send(String::from("Pesan dari handler1")).unwrap();
        });

        let handler2 = thread::spawn(move || {
            println!("[Sender / handler2] Mengirim pesan");
            sender2.send(String::from("Pesan dari handler2")).unwrap();
        });

        let handler3 = thread::spawn(move || {
            println!("[Receiver / handler3] Thread dimulai");

            for received in receiver {
                println!("[Receiver / handler3] Menerima: {}", received);
            }

            println!("[Receiver / handler3] Channel ditutup, receiver selesai");
        });

        handler1.join().unwrap();
        handler2.join().unwrap();
        handler3.join().unwrap();

        println!("=== selesai multiple_senders_with_handler ===\n");
    }
    #[test]
    fn test_multiple_senders_with_handler() {
        multiple_senders_with_handler();
    }

    // ## Race condition
    static mut COUNTER: i32 = 0;
    #[test]
    fn race_condition() {
        let mut handlers = vec![];
        for _ in 0..10 {
            let handler = thread::spawn(|| unsafe {
                for j in 0..1000000 {
                    COUNTER += 1; // ini akan race condition untuk setiap thread
                }
            });
            handlers.push(handler);
        }

        for handler in handlers {
            handler.join().unwrap()
        }
        println!("counter: {}", unsafe { COUNTER })
    }

    // ## Atomic
    #[test]
    fn test_atomic() {
        use std::sync::atomic::{AtomicI32, Ordering};
        // static counter: AtomicI32 = AtomicI32::new(0);
        static COUNTER: AtomicI32 = AtomicI32::new(0);

        let mut handlers = vec![];
        for _ in 0..10 {
            let handler = thread::spawn(move || {
                for _2 in 0..1000000 {
                    COUNTER.fetch_add(1, Ordering::Relaxed); // atomic menjamin tidak akan race condition
                }
            });
            handlers.push(handler);
        }
        for handler in handlers {
            handler.join().unwrap();
        }
        println!("COUNTER: {}", COUNTER.load(Ordering::Relaxed));
    }

    // ## Atomic Reference (Arc)
    #[test]
    fn test_atomic_reference() {
        use atomic::{AtomicI32, Ordering};
        use std::sync::{Arc, atomic};

        let counter: Arc<AtomicI32> = Arc::new(AtomicI32::new(0));
        let mut handlers = vec![];
        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            let handler = thread::spawn(move || {
                for _2 in 0..1000000 {
                    counter_clone.fetch_add(1, Ordering::Relaxed);
                }
            });
            handlers.push(handler);
        }
        for handler in handlers {
            handler.join().unwrap();
        }
        println!("counter: {}", counter.load(Ordering::Relaxed));
    }

    // ## Mutex
    /*
    # Mutex
    - Mutex adalah Mutual Exclusion, yaitu tipe data yang digunakan untuk melindungi data yang di-sharing ke lebih dari satu thread
    - Mutex akan memblok thread dan menunggu sampai lock (kunci) tersedia
    - Kita bisa menggunakan method lock() pada Mutex untuk menunggu sampai mendapatkan data, dan setelah data keluar dari scope,
    maka lock (kunci) akan dikembalikan ke Mutex sehingga thread lain bisa mengambil lock (kunci) nya
    - https://doc.rust-lang.org/std/sync/struct.Mutex.html


    */
    #[test]
    fn test_mutex() {
        use std::sync::{Arc, Mutex, MutexGuard};
        let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
        let mut handlers = vec![];
        for _1 in 0..10 {
            let counter_clone = Arc::clone(&counter);
            let handler = thread::spawn(move || {
                for _2 in 0..1000000 {
                    let mut data: MutexGuard<i32> = counter_clone.lock().unwrap();
                    *data += 1;
                }
                // data akan di unlock secara otomatis setelah keluar dari scope
            });
            handlers.push(handler);
        }
        for handler in handlers {
            handler.join().unwrap();
        }
        println!("counter: {}", *counter.lock().unwrap());
    }

    // pembatas
}
