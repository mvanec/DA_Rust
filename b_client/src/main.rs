// Import necessary libraries
use std::net::TcpStream;
use std::io::{Read, Write};
use std::thread;

// Define the number of threads
const NUM_THREADS: usize = 10;

fn main() -> std::io::Result<()> {
    // Create a vector to store thread handles
    let mut handles = Vec::new();

    // Create a specified number of threads
    for i in 0..NUM_THREADS {
        let i = i.clone();
        // Spawn a new thread
        let handle = thread::spawn(move || {
            // Attempt to connect to the server
            match TcpStream::connect("127.0.0.1:7878") {
                Ok(mut stream) => {
                    // Create a message to send to the server
                    let message = format!("Client thread {}", i);
                    // Send the message
                    stream.write(message.as_bytes()).unwrap();
                    stream.flush().unwrap();

                    // Create a buffer to store the response
                    let mut buffer = [0; 512];
                    // Read the response from the server
                    stream.read(&mut buffer).unwrap();
                    // Convert the response to a string
                    let response = String::from_utf8_lossy(&buffer).trim().to_string();
                    // Print the response
                    println!("Client received: {}", response);
                },
                Err(e) => {
                    // Print an error message if the connection fails
                    println!("Error connecting to server: {}", e);
                }
            }
        });
        // Store the thread handle
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}
