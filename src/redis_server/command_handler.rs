use super::request::Request;
use crate::redis_database::SharedDatabase;
use anyhow::{bail, Result};

pub fn handle_request(request: Request, database: SharedDatabase) -> Result<String> {
    let commands = Commands::new();

    let handler = commands
        .handlers
        .iter()
        .find(|h| h.command() == request.command);

    if let Some(handler) = handler {
        handler.execute(&request.args, database)
    } else {
        bail!("unsupportd command: {}", request.command)
    }
}

struct Commands {
    handlers: Vec<Box<dyn CommandHandler>>,
}

impl Commands {
    fn new() -> Self {
        Self {
            handlers: vec![
                Box::new(PingHandler),
                Box::new(EchoHandler),
                Box::new(ClientHandler),
                Box::new(SetHandler),
                Box::new(GetHandler),
            ],
        }
    }
}

trait CommandHandler {
    fn command(&self) -> &str;
    fn execute(&self, args: &[String], db: SharedDatabase) -> Result<String>;
}

struct PingHandler;

impl CommandHandler for PingHandler {
    fn command(&self) -> &str {
        "PING"
    }

    fn execute(&self, _: &[String], _: SharedDatabase) -> Result<String> {
        Ok("+PONG\r\n".to_string())
    }
}

struct EchoHandler;

impl CommandHandler for EchoHandler {
    fn command(&self) -> &str {
        "ECHO"
    }

    fn execute(&self, args: &[String], _: SharedDatabase) -> Result<String> {
        if args.len() != 1 {
            bail!("Invalid number of arguments")
        }

        Ok(format!("${}\r\n{}\r\n", args[0].len(), args[0]))
    }
}

struct ClientHandler;

impl CommandHandler for ClientHandler {
    fn command(&self) -> &str {
        "CLIENT"
    }

    fn execute(&self, _: &[String], _: SharedDatabase) -> Result<String> {
        Ok("+OK\r\n".to_string())
    }
}

struct SetHandler;

impl CommandHandler for SetHandler {
    fn command(&self) -> &str {
        "SET"
    }

    fn execute(&self, args: &[String], mut database: SharedDatabase) -> Result<String> {
        if args.len() != 2 {
            bail!("Invalid number of arguments")
        }

        _ = database.set(args[0].clone(), args[1].clone());

        Ok("+OK\r\n".to_string())
    }
}

struct GetHandler;

impl CommandHandler for GetHandler {
    fn command(&self) -> &str {
        "GET"
    }

    fn execute(&self, args: &[String], database: SharedDatabase) -> Result<String> {
        if args.len() != 1 {
            bail!("Invalid number of arguments")
        }

        let value = database.get(args[0].clone()).unwrap_or_default();
        Ok(format!("${}\r\n{}\r\n", value.len(), value))
    }
}
