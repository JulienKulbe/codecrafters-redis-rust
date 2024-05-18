use anyhow::{bail, Result};
use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
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
    let mut writer = BufWriter::new(stream.try_clone()?);
    let mut reader = BufReader::new(&mut stream);

    loop {
        let mut line = String::new();
        let received = reader.read_line(&mut line)?;
        if received == 0 {
            //return Ok(());
            thread::sleep(Duration::from_millis(100));
            continue;
        }

        let mut request = Vec::new();
        line = line.trim_end().to_string();
        if let Some(line_count) = line.strip_prefix('*') {
            let line_count = line_count.parse::<usize>()?;
            for _ in 0..2 * line_count {
                let mut line = String::new();
                reader.read_line(&mut line)?;
                line = line.trim_end().to_string();
                request.push(line);
            }
        }

        if request.is_empty() || request.len() % 2 != 0 || request.len() < 2 {
            bail!(
                "Invalid line count, expected number of lines: {}, received {}",
                line,
                request.len()
            );
        }

        let command = request[1].to_uppercase();
        let response = match command.as_str() {
            "PING" => "+PONG\r\n".to_string(),
            "ECHO" => format!("$3\r\n{}\r\n", request[3]),
            _ => bail!("unsupportd command: {}", command),
        };

        writer.write_all(response.as_bytes())?;
    }
}
