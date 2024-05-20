use super::{request::Request, response::Response};
use crate::redis_database::SharedDatabase;
use anyhow::{bail, Result};

pub fn handle_request(request: Request, database: SharedDatabase) -> Result<Response> {
    let command = request.command.clone();
    match command.as_str() {
        "PING" => ping(),
        "ECHO" => echo(&request.args),
        "CLIENT" => client(),
        "SET" => set_data(&request, database),
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
    // TODO ignore client args for now and just respond with ok
    Ok(Response::ok())
}

fn set_data(request: &Request, mut database: SharedDatabase) -> Result<Response> {
    if request.args.len() < 2 {
        bail!("Invalid number of arguments")
    }

    let px = request.get_argument_value("PX");
    _ = database.set(&request.args[0], &request.args[1], px);

    Ok(Response::ok())
}

fn get_data(args: &[String], mut database: SharedDatabase) -> Result<Response> {
    if args.len() != 1 {
        bail!("Invalid number of arguments")
    }

    let value = database.get(&args[0]).unwrap_or_default();
    Ok(Response::bulk(&value))
}
