use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    
    while match stream.read(&mut buffer) {
        Ok(size) if size > 0 => {
            let received_data = String::from_utf8_lossy(&buffer[..size]);
            println!("Received: {}", received_data);
            
            true
        },
        _ => false,
    } {}
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:873").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => eprintln!("Failed to accept connection: {:?}", e),
        }
    }
}
