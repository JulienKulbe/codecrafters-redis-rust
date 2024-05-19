use super::{request::Request, response::Response};
use crate::redis_database::SharedDatabase;
use anyhow::{bail, Result};

pub fn handle_request(request: Request, database: SharedDatabase) -> Result<Response> {
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

fn ping() -> Result<Response> {
    Ok(Response::simple("PONG"))
}

fn echo(args: &[String]) -> Result<Response> {
    if args.len() != 1 {
        bail!("Invalid number of arguments")
    }

    Ok(Response::bulk(&args[0]))
}

fn client() -> Result<Response> {
    Ok(Response::ok())
}

fn set_data(args: &[String], mut database: SharedDatabase) -> Result<Response> {
    if args.len() != 2 {
        bail!("Invalid number of arguments")
    }

    _ = database.set(args[0].clone(), args[1].clone());

    Ok(Response::ok())
}

fn get_data(args: &[String], database: SharedDatabase) -> Result<Response> {
    if args.len() != 1 {
        bail!("Invalid number of arguments")
    }

    let value = database.get(args[0].clone()).unwrap_or_default();
    Ok(Response::bulk(&value))
}
