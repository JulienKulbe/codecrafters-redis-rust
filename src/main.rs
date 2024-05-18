use anyhow::{bail, Result};
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                thread::spawn(|| {
                    handle_connection(stream).unwrap();
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    loop {
        let mut buffer = [0_u8; 512];

        let bytes = stream.read(&mut buffer)?;
        let request = String::from_utf8_lossy(&buffer);
        println!(">> {request} ({bytes})");

        let request = request.split("\r\n").collect::<Vec<&str>>();

        // TODO verify request len

        let command = request[2].to_uppercase();
        let response = match command.as_str() {
            "PING" => "+PONG\r\n".to_string(),
            "ECHO" => format!("$3\r\n{}\r\n", request[3]),
            "CLIENT" => "+OK\r\n".to_string(),
            _ => bail!("unsupportd command: {}", command),
        };

        stream.write_all(response.as_bytes())?;
    }
}
