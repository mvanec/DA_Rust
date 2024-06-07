// Import the necessary modules from the Rust standard library.
// We're using the mpsc (multi-producer, single-consumer) module for message passing,
// and the thread module for creating and managing threads.
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

// Define a constant for the number of threads we want to create.
static NTHREADS: i32 = 8;

// Define a callback function that will be executed by each thread.
// This function is a placeholder for some actual work that you'd want each thread to do.
// In this case, it's just printing some data to the console.
fn callback(count: i32, data: Vec<String>, columns: Vec<String>) {
    // This callback will be executed for each row from the SELECT
    for i in 0..count {
        println!("{}: {}", columns[i as usize], data[i as usize]);
    }
    println!();
}

// Define the main function, which is the entry point for our program.
fn main() {
    // Create a channel for sending and receiving messages between threads.
    // The channel will be used to send IDs from the child threads back to the main thread.
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    // Create a vector to store the handles of the child threads.
    let mut children = Vec::new();

    // Create NTHREADS child threads.
    for id in 0..NTHREADS {
        // Clone the sender end of the channel, so each thread can send messages.
        let thread_tx = tx.clone();

        // Spawn a new child thread.
        let child = thread::spawn(move || {
            // Send the thread's ID back to the main thread via the channel.
            thread_tx.send(id).unwrap();

            // Call the callback function to do some actual work.
            callback(2, vec!["INTEGER".to_string(), "TEXT".to_string()], vec!["id".to_string(), "name".to_string()]);

            // Print a message to indicate that the thread has finished its work.
            println!("thread {} finished", id);
        });

        // Store the child thread's handle in the vector.
        children.push(child);
    }

    // Here, all the messages are collected from the child threads.
    // We're receiving the IDs sent by the child threads and storing them in a vector.
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        ids.push(rx.recv().unwrap());
    }

    // Wait for the child threads to complete any remaining work.
    // This is necessary because the child threads might not have finished yet,
    // even though they've sent their IDs back to the main thread.
    for child in children {
        child.join().unwrap();
    }

    // Finally, print the IDs received from the child threads.
    println!("{:?}", ids);
}
