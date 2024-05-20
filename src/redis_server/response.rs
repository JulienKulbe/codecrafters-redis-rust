pub enum Response {
    SimpleString(String),
    BulkString(String),
    NullBulkString,
}

impl Response {
    pub fn ok() -> Self {
        Self::SimpleString("OK".to_string())
    }

    pub fn simple(str: &str) -> Self {
        Self::SimpleString(str.to_string())
    }

    pub fn bulk(str: &str) -> Self {
        Self::BulkString(str.to_string())
    }
}

impl From<Response> for String {
    fn from(val: Response) -> Self {
        match val {
            Response::SimpleString(str) => format!("+{str}\r\n"),
            Response::BulkString(str) => format!("${}\r\n{}\r\n", str.len(), str),
            Response::NullBulkString => "$-1\r\n".to_string(),
        }
    }
}
