use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::SystemTime;

// Logging Functionality
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut log_file = File::create("log.txt").expect("Failed to create log file");

    // Get the source IP address of the client
    let source_ip = stream
        .peer_addr()
        .map(|addr| addr.ip().to_string())
        .unwrap_or_else(|_| "Unknown".to_string());

    // Get the current timestamp
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    loop {
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    break;
                }

                // Process the received data or log it to a file
                let received_data = &buffer[..n];
                let received_str = std::str::from_utf8(received_data).expect("Invalid UTF-8 data");

                // Create the log message with source IP and timestamp
                let log_message = format!(
                    "Source IP: {} | Timestamp: {} | Data: {}",
                    source_ip, timestamp, received_str
                );

                // Log the message to a file
                log_file
                    .write_all(log_message.as_bytes())
                    .expect("Failed to write to log file");

                buffer = [0; 1024];
            }
            Err(e) => {
                eprintln!("Error reading from stream: {}", e);
                break;
            }
        }
    }

    // Clean up resources
    log_file.flush().expect("Failed to flush log file");
    log_file.sync_all().expect("Failed to sync log file");
}

// TCP Server start Functionality
fn start_server() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a new thread or async task to handle each client connection
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}

fn main() {
    println!("Hello, world!");
    match start_server() {
        Ok(()) => {
            println!("TCP server started successfully.");
        }
        Err(err) => {
            eprintln!("Failed to start TCP server: {}", err);
        }
    }
}
