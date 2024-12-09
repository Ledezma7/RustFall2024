use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    
    let mut handles = vec![];

    
    for _ in 0..5 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                // Locks the mutex to safely update the counter
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }

    // Waits for all the threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Prints the final value of the counter
    println!("Final value of counter: {}", *counter.lock().unwrap());
}
