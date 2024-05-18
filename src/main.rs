use redis_database::SharedDatabase;

use crate::redis_server::handle_connection;
use std::{net::TcpListener, thread};

mod redis_database;
mod redis_server;

fn main() {
    let database = SharedDatabase::new();
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                let db = database.clone();
                thread::spawn(|| {
                    handle_connection(stream, db).unwrap();
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
