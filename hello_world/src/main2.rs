use std::thread;

fn main() {
    // Create a vector to hold the thread handles
    let mut handles = Vec::new();

    for i in 1..=3 {
        // Spawn a thread and move the identifier into the thread
        let handle = thread::spawn(move || {
            println!("Thread {}", i);
        });

        // Store the thread handle
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread failed to join.");
    }

    // Print a message after all threads are done
    println!("All threads completed.");
}
