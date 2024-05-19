use super::request::Request;
use crate::redis_database::SharedDatabase;
use anyhow::{bail, Result};

pub fn handle_request(request: Request, database: SharedDatabase) -> Result<String> {
    let command = request.command.to_ascii_uppercase();
    match command.as_str() {
        "PING" => ping(),
        "ECHO" => echo(&request.args),
        "CLIENT" => client(),
        "SET" => set_data(&request.args, database),
        "GET" => get_data(&request.args, database),
        _ => bail!("unsupportd command: {}", command),
    }
}

fn ping() -> Result<String> {
    Ok("+PONG\r\n".to_string())
}

fn echo(args: &[String]) -> Result<String> {
    if args.len() != 1 {
        bail!("Invalid number of arguments")
    }

    Ok(format!("${}\r\n{}\r\n", args[0].len(), args[0]))
}

fn client() -> Result<String> {
    Ok("+OK\r\n".to_string())
}

fn set_data(args: &[String], mut database: SharedDatabase) -> Result<String> {
    if args.len() != 2 {
        bail!("Invalid number of arguments")
    }

    _ = database.set(args[0].clone(), args[1].clone());

    Ok("+OK\r\n".to_string())
}

fn get_data(args: &[String], database: SharedDatabase) -> Result<String> {
    if args.len() != 1 {
        bail!("Invalid number of arguments")
    }

    let value = database.get(args[0].clone()).unwrap_or_default();
    Ok(format!("${}\r\n{}\r\n", value.len(), value))
}
