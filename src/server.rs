use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::thread;
use std::env;

pub fn serve() {
    let port = match env::var("PORT") {
        Ok(val) => val,
        Err(_) => "3333".to_string(),
    };

    let host = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(host).unwrap();
    println!("Listening on port {}", port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream)
                });
            },
            Err(e) => {
                println!("Unable to connect: {}", e);
            },
        }
    }
}

fn handle_client(stream: TcpStream) {
    handle_read(&stream);
    handle_write(stream);
}

fn handle_read(mut stream: &TcpStream) {
    let mut buf = [0u8; 4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buf);
            println!("{}", req_str);
        },
        Err(e) => println!("Unable to read stream: {}", e),
    }
}

fn handle_write(mut stream: TcpStream) {
    // TODO: event emitter
    // TODO: success response

    let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";

    match stream.write(response) {
        Ok(_) => println!("response sent"),
        Err(e) => println!("failed to send response: {}", e),
    }
}
