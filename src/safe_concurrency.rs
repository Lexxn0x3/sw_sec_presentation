use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::{thread_rng, Rng}; // Import random number generator utilities

pub fn create_increment_threads(){
    let counter = Arc::new(Mutex::new(0));
    let mut children = vec![];

    for _ in 0..5 {
        let counter_clone = Arc::clone(&counter);
        let child = thread::spawn(move || {
            let mut rng = thread_rng(); // Create a random number generator
            for _ in 0..10 { // Loop to add to the counter 10 times
                {
                    let mut num = counter_clone.lock().unwrap();
                    *num += 1;
                } // MutexGuard goes out of scope here, releasing the lock

                let sleep_time = rng.gen_range(1..=3); // Generate a random number between 1 and 4
                thread::sleep(Duration::from_millis(sleep_time)); // Sleep for the random duration
            }
        });
        children.push(child);
    }

    // Wait for all threads to complete
    for child in children {
        child.join().unwrap();
    }

    // Print the result
    println!("Final counter value: {}", *counter.lock().unwrap());
}
pub fn how_not_to_create_increment_threads(){
    /*
    let mut counter = 0;
    //need to use a refence otherwise int will be copied for each thread.
    let counter_ref = &mut counter; // Explicit mutable reference to `counter`

    let mut handles = vec![];

    for _ in 0..10 {
        // Attempt to use the mutable reference in multiple threads
        let handle = thread::spawn(move || {
            *counter_ref += 1; // Error: `counter_ref` cannot be sent safely between threads
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Counter: {}", counter);
    */
}