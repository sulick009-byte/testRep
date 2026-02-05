use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("failed to bind to 127.0.0.1:8080");
    println!("Server running at http://127.0.0.1:8080");

    let body = format!("[{}]", vec!["0"; 1000].join(","));
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.len(),
        body
    );

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0_u8; 1024];
                let _ = stream.read(&mut buffer);
                if let Err(err) = stream.write_all(response.as_bytes()) {
                    eprintln!("failed to write response: {err}");
                }
            }
            Err(err) => eprintln!("connection failed: {err}"),
        }
    }
}
