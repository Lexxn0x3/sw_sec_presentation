use std::sync::mpsc;
use std::thread;

pub fn iterator(){
    let numbers = vec![1, 2, 3, 4, 5];

    // Using an iterator to iterate through the vector safely
    for num in numbers.iter() {
        println!("{}", num); // Safely prints each number
    }
}
pub fn message_passing(){
    let (tx, rx) = mpsc::channel();

    let sender_thread = thread::spawn(move || {
        let msg = "Hello from the sender thread";
        tx.send(msg).unwrap(); // Sends a message safely to the receiver
        println!("Sent message: '{}'", msg);
    });

    let receiver_thread = thread::spawn(move || {
        let received = rx.recv().unwrap(); // Receives message safely
        println!("Received message: '{}'", received);
    });

    sender_thread.join().unwrap();
    receiver_thread.join().unwrap();
}
fn find_divisor(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        None // Safely handle division by zero
    } else {
        Some(dividend / divisor) // Safely return the result
    }
}
pub fn option_type(){
    let dividend = 10;
    let divisor = 0;

    match find_divisor(dividend, divisor) {
        Some(result) => println!("Result of division: {}", result),
        None => println!("Cannot divide by zero!"),
    }
}

pub fn filter(){
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Use filter to find even numbers
    let even_numbers: Vec<_> = numbers.iter()
        .filter(|&&x| x % 2 == 0)
        .collect();

    println!("Even numbers: {:?}", even_numbers);
}