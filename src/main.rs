use anyhow::Result;
use std::{
    io::{Read, Write},
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
    // let buf_reader = BufReader::new(&mut stream);
    // let request_lines = buf_reader.lines();
    // for request_line in request_lines.map_while(Result::ok) {
    //     println!("{request_line}");
    // }

    let mut buffer = [0_u8; 256];
    let _ = stream.read(&mut buffer)?;
    let request = String::from_utf8_lossy(&buffer);
    println!(">> {request}");

    let response = "+PONG\r\n";
    println!("<< {response}");

    stream.write_all(response.as_bytes())?;

    Ok(())
}
