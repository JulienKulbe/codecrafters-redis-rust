use anyhow::{bail, Result};
use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::redis_server::request::Request;

pub mod request;
pub mod response;

pub fn handle_connection(mut stream: TcpStream) -> Result<()> {
    loop {
        let request = Request::new(&mut stream)?;

        let response = match request.command.to_uppercase().as_str() {
            "PING" => "+PONG\r\n".to_string(),
            "ECHO" => format!("${}\r\n{}\r\n", request.args[0].len(), request.args[0]),
            "CLIENT" => "+OK\r\n".to_string(),
            _ => bail!("unsupportd command: {}", request.command),
        };

        println!("<< :{response}");
        stream.write_all(response.as_bytes())?;
    }
}
