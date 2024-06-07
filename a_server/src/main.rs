// Import necessary libraries
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::process;
use ctrlc;

// Define the maximum number of threads
const MAX_THREADS: usize = 5;

// Function to handle incoming requests
fn handle_request(mut stream: TcpStream, id: usize) {
    // Create a buffer to store incoming data
    let mut buffer = [0; 512];
    // Read data from the stream
    stream.read(&mut buffer).unwrap();
    // Convert the data to a string
    let message = String::from_utf8_lossy(&buffer).trim().to_string();

    // Print the received message
    println!("Thread {} received: {}", id, message);

    // Create a response
    let response = format!("Thread {} received: {}", id, message);
    // Send the response back to the client
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() -> std::io::Result<()> {
    // Create a TCP listener
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    // Create a counter to keep track of the number of threads
    let counter = Arc::new(Mutex::new(0));

    // Set up a handler for Ctrl-C
    let _ = ctrlc::set_handler(move || {
        println!("Exiting...");
        process::exit(0);
    }).unwrap();

    // Create a specified number of threads
    for _ in 0..MAX_THREADS {
        let counter = Arc::clone(&counter);
        let listener = listener.try_clone()?;
        // Spawn a new thread
        thread::spawn(move || {
            loop {
                // Accept incoming connections
                let (stream, _) = listener.accept().unwrap();
                // Increment the counter
                *counter.lock().unwrap() += 1;

                // Get the current thread ID
                let id = *counter.lock().unwrap();
                // Handle the incoming request
                handle_request(stream, id);

                // Decrement the counter
                *counter.lock().unwrap() -= 1;
            }
        });
    }

    // Main loop to handle incoming connections
    loop {
        let (mut stream, _) = listener.accept()?;
        // Check if the maximum number of threads has been reached
        if *counter.lock().unwrap() >= MAX_THREADS {
            // Send a "BUSY" response if the server is busy
            stream.write("BUSY".as_bytes())?;
            stream.flush()?;
        }
    }
}
