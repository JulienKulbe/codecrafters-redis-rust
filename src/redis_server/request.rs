use anyhow::{bail, Context, Ok, Result};
use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};

pub struct Request {
    pub command: String,
    pub args: Vec<String>,
}

impl Request {
    pub fn new(stream: &mut TcpStream) -> Result<Self> {
        // *2\r\n$4\r\nECHO\r\n$3\r\nhey\r\n
        let mut reader = BufReader::new(stream);

        let lines = Self::get_number_of_lines(&mut reader)?;
        if lines < 1 {
            bail!("Expected minimum of 1 line");
        }

        let command = Self::read_line_pair_with_size_and_value(&mut reader)?;

        let mut args = Vec::new();
        for _ in 1..lines {
            let arg = Self::read_line_pair_with_size_and_value(&mut reader)?;
            args.push(arg);
        }

        Ok(Self { command, args })
    }

    fn get_number_of_lines(reader: &mut BufReader<&mut TcpStream>) -> Result<usize> {
        Ok(Self::read_line(reader)?
            .strip_prefix('*')
            .context("Invalid start of line")?
            .parse::<usize>()?)
    }

    fn read_line(reader: &mut BufReader<&mut TcpStream>) -> Result<String> {
        let mut buffer = String::new();
        _ = reader.read_line(&mut buffer)?;

        Ok(buffer
            .strip_suffix("\r\n")
            .expect("Indalid line end")
            .to_string())
    }

    fn read_line_pair_with_size_and_value(
        reader: &mut BufReader<&mut TcpStream>,
    ) -> Result<String> {
        let length = Self::read_line(reader)?
            .strip_prefix('$')
            .context("Invalid size")?
            .parse::<usize>()?;

        let value = Self::read_line(reader)?;
        if value.len() != length {
            bail!("Indalid command length");
        }

        Ok(value)
    }
}
