use crate::http::method;

use super::method::{Method, MethodError};
use super::QueryString;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::Utf8Error;
use std::{str, string};

#[derive(Debug)]
pub struct Request<'buffer> {
    path: &'buffer str,
    query_string: Option<QueryString<'buffer>>,
    method: Method,
}

impl<'buffer> Request<'buffer> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
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

impl Error for ParseError {}

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

impl<'buffer> TryFrom<&'buffer [u8]> for Request<'buffer> {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buffer: &'buffer [u8]) -> Result<Request<'buffer>, Self::Error> {
        // match str::from_utf8(buffer) {
        //     Ok(request) => {}
        //     Err(_) => return Err(ParseError::InvalidEncoding),
        // }

        // match str::from_utf8(buffer).or(Err(ParseError::InvalidEncoding)) {
        //     Ok(request) => {}
        //     Err(e) => return Err(e),
        // }

        //it the same of the previous part
        // let request = str::from_utf8(buffer).or(Err(ParseError::InvalidEncoding))?;

        // grace a la ligne 68
        let request = str::from_utf8(buffer)?;
        // match get_next_word(request) {
        //     Some((method, request)) => {}
        //     None => return Err(ParseError::InvalidRequest),
        // }

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;
        let mut query_string = None;
        // match path.find('?') {
        //     Some(i) => {
        //         query_String = Some(&path[i + 1..]);
        //         path = &path[..i];
        //     }
        //     None => {}
        // }

        // let q = path.find('?');
        // if q.is_some(){
        //     let i = q.unwrap();
        //     query_String = Some(&path[i + 1..]);
        //     path = &path[..i];
        // }

        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    let mut iterator = request.chars();
    // loop {
    //     let item = iterator.next();
    //     match item {
    //         Some(c) => {}
    //         None => break,
    //     }
    // }

    for (index, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..index], &request[index + 1..]));
        }
    }
    None
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}
// impl TryFrom<&[u8]> for Request {
//     type Error = String;
//     fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
//         // let string = String::from("asd");
//         // string.encrypt();
//         unimplemented!()
//     }
// }

// trait Encrypt {
//     fn encrypt(&self) -> Self;
// }

// impl Encrypt for String {
//     fn encrypt(&self) -> Self {
//         unimplemented!()
//     }
// }
