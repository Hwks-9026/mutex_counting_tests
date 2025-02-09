use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles: Vec<_> = (0..30).map(|_| {
        let counter = Arc::clone(&counter); // Clone Arc for each thread
        thread::spawn(move || {
            for _ in 0..1000000 {
                let mut num = counter.lock().unwrap(); // Lock mutex before modifying
                *num += 1;
            }
        })
    })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Final counter value: {}", *counter.lock().unwrap());
}
