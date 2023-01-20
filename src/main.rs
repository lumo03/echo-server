use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn handle_client(mut stream: TcpStream) {
    println!(
        "Client connected with ip '{}'.",
        stream.peer_addr().unwrap()
    );
    loop {
        let mut read = [0; 1028];
        match stream.read(&mut read) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                stream.write_all(&read[0..n]).unwrap();
            }
            Err(err) => panic!("Error: {}", err),
        }
    }
    println!(
        "Client with ip '{}' disconnected.",
        stream.peer_addr().unwrap()
    );
}
fn main() {
    println!("Echo server running...");
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(_) => panic!("An error occured."),
        }
    }
}
