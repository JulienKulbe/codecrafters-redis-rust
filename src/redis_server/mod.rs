use anyhow::Result;
use std::{io::Write, net::TcpStream};

use crate::{
    redis_database::SharedDatabase,
    redis_server::{command_handler::handle_request, request::Request},
};

pub mod command_handler;
pub mod request;
pub mod response;

pub fn handle_connection(mut stream: TcpStream, database: SharedDatabase) -> Result<()> {
    loop {
        let request = Request::new(&mut stream)?;
        let response = handle_request(request, database.clone())?;
        stream.write_all(response.as_bytes())?;
    }
}
