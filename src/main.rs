use anyhow::Result;
use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                handle_connection(stream).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    // let mut buffer = [0_u8; 12];

    // let bytes = stream.read(&mut buffer)?;
    // let request = String::from_utf8_lossy(&buffer);
    // println!(">> {request} ({bytes})");

    let mut reader = BufReader::new(&mut stream);
    let received: Vec<u8> = reader.fill_buf()?.to_vec();

    let request = String::from_utf8_lossy(&received);
    println!(">> {request} ({})", received.len());

    reader.consume(received.len());

    let response = "+PONG\r\n";
    println!("<< {response}");

    stream.write_all(response.as_bytes())?;

    Ok(())
}
