use anyhow::{bail, Context, Ok, Result};
use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
    str::FromStr,
};

pub struct Request {
    pub command: String,
    pub args: Vec<String>,
}

impl Request {
    pub fn new(stream: &mut TcpStream) -> Result<Self> {
        // Example request:
        // *2\r\n$4\r\nECHO\r\n$3\r\nhey\r\n
        let mut reader = RequestParser::new(stream);

        let lines = reader.get_number_of_lines()?;
        if lines < 1 {
            bail!("Expected minimum of 1 line");
        }

        let command = reader
            .read_line_pair_with_size_and_value()?
            .to_ascii_uppercase();

        let mut args = Vec::new();
        for _ in 1..lines {
            let arg = reader.read_line_pair_with_size_and_value()?;
            args.push(arg);
        }

        Ok(Self { command, args })
    }

    pub fn get_argument_value<T>(&self, argument: &str) -> Option<T>
    where
        T: FromStr,
    {
        let argument = argument.to_ascii_uppercase();
        let pos = self
            .args
            .iter()
            .position(|a| a.to_ascii_uppercase() == argument);

        if let Some(pos) = pos {
            let value_pos = pos + 1;
            if value_pos < self.args.len() {
                let value = self.args[value_pos].parse::<T>().ok()?;
                return Some(value);
            }
        }

        None
    }
}

struct RequestParser<'a> {
    reader: BufReader<&'a mut TcpStream>,
}

impl<'a> RequestParser<'a> {
    fn new(stream: &'a mut TcpStream) -> Self {
        Self {
            reader: BufReader::new(stream),
        }
    }

    fn get_number_of_lines(&mut self) -> Result<usize> {
        Ok(self
            .read_line()?
            .strip_prefix('*')
            .context("Invalid start of line")?
            .parse::<usize>()?)
    }

    fn read_line_pair_with_size_and_value(&mut self) -> Result<String> {
        let length = self
            .read_line()?
            .strip_prefix('$')
            .context("Invalid line size")?
            .parse::<usize>()?;

        let value = self.read_line()?;
        if value.len() != length {
            bail!("Indalid line length");
        }

        Ok(value)
    }

    fn read_line(&mut self) -> Result<String> {
        let mut buffer = String::new();
        _ = self.reader.read_line(&mut buffer)?;

        Ok(buffer
            .strip_suffix("\r\n")
            .expect("Indalid line end")
            .to_string())
    }
}
