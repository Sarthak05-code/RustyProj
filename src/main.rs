#![allow(unused)]
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn handle_client(mut stream: TcpStream) {
    let buffer_reader = BufReader::new(&stream);

    if let Some(Ok(request_line)) = buffer_reader.lines().next() {
        println!("Received HTTP Request: {request_line}");
    }
    let status_code = "HTTP/1.1 200 OK";
    let content = "<h1> Hello from your RUST TCP Server!. </h1>";
    let length = content.len();

    let response = format!(
        "{}\r\nContent-length: {}\r\n\r\n{}",
        status_code, length, content
    );

    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to send response : {e}");
    }
}

fn main() -> std::io::Result<()> {
    let socket = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server is listening: 8080...");
    for stream in socket.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error : {e}")
            }
        }
    }

    Ok(())
}
