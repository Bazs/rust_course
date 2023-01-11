use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::{self, Utf8Error};

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?;

        match get_next_word(request) {
            Some((method, request)) => {}
            None => return Err(ParseError::InvalidRequest),
        }

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidEncoding)?;
        let (path, request) = get_next_word(request).ok_or(ParseError::InvalidEncoding)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidEncoding)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        unimplemented!()
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    let mut iter = request.chars();

    for (idx, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..idx], &request[idx + 1..]));
        }
    }

    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(value: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}
