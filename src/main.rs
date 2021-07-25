use std::thread;
use std::io::{Read, Write};
use std::net::{ TcpListener, TcpStream, Shutdown };

fn handle_client(mut stream: TcpStream) {
    // using 50 byte buffer
    let mut data = [0 as u8; 50];
    loop {
        match stream.read(&mut data) {
            Ok(size) => {
                // echo everything
                stream.write(&data[0..size]).unwrap();
            },
            Err(_) => {
                println!("An error occurred.");
                stream.shutdown(Shutdown::Both).unwrap();
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    println!("Server listening on port 7878");

    // accept connections and process them serially
    for stream in listener.incoming() {
        // using `match` to handle stream connection exception.
        match stream {
            Ok(stream) => {
                // create a thread and give a ownership of values to this thread using move.
                thread::spawn(move|| {
                    // handle the input of client
                    handle_client(stream);
                });
            }
            Err(e) => {
                // print error message
                println!("Error: connection failed. {}", e);
            }
        }
    }
    // close the socket server
    drop(listener);
}
