use super::request::Request;
use anyhow::{bail, Result};

pub fn handle_request(request: Request) -> Result<String> {
    let commands = Commands::new();

    let handler = commands
        .handlers
        .iter()
        .find(|h| h.command() == request.command);

    if let Some(handler) = handler {
        handler.execute(&request.args)
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
            ],
        }
    }
}

trait CommandHandler {
    fn command(&self) -> &str;
    fn execute(&self, args: &[String]) -> Result<String>;
}

struct PingHandler;

impl CommandHandler for PingHandler {
    fn command(&self) -> &str {
        "PING"
    }

    fn execute(&self, _: &[String]) -> Result<String> {
        Ok("+PONG\r\n".to_string())
    }
}

struct EchoHandler;

impl CommandHandler for EchoHandler {
    fn command(&self) -> &str {
        "ECHO"
    }

    fn execute(&self, args: &[String]) -> Result<String> {
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

    fn execute(&self, _: &[String]) -> Result<String> {
        Ok("+OK\r\n".to_string())
    }
}

struct SetHandler;

impl CommandHandler for SetHandler {
    fn command(&self) -> &str {
        "SET"
    }

    fn execute(&self, _: &[String]) -> Result<String> {
        Ok("+OK\r\n".to_string())
    }
}
