use crate::http::{status_code, ParseError};
use crate::http::{Request, Response, StatusCode};
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::{Read, Write};
use std::net::TcpListener;

pub trait Handler {
    fn handler_request(&mut self, request: &Request) -> Response;
    fn handler_bad_request(&mut self, e: &ParseError) -> Response {
        println!(" Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    handler.handler_request(&request)
                                    // write!(stream, "{}", response);
                                }
                                Err(e) => handler.handler_bad_request(&e),
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response {}", e);
                            }
                            // let result: &Result<Request, _> = &buffer[..].try_into();
                        }
                        Err(e) => println!("Fail to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Fail to establish a connection: {}", e),
            }
        }
    }
}
