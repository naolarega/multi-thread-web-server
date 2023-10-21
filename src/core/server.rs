use std::{
    collections::HashMap,
    fmt::Display,
    io::{Read, Write},
    net::{TcpListener, TcpStream}
};

use super::parser;

pub struct Server {
    path_handlers: HashMap<&'static str, (HttpMethod, fn(Request, Response))>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            path_handlers: HashMap::new(),
        }
    }

    pub fn add_router(
        &mut self,
        method: HttpMethod,
        path: &'static str,
        handler: fn(Request, Response)
    ) {
        self.path_handlers.insert(path, (method, handler));
    }

    pub fn serve(&mut self, host: &str) -> Result<(), String> {
        if let Ok(tcp_listener) = TcpListener::bind(host) {
            for tcp_stream in tcp_listener.incoming() {
                if let Ok(mut tcp_stream) = tcp_stream {
                    let mut buffer = String::new();

                    tcp_stream.read_to_string(&mut buffer);

                    let request = parser::parse_http_request(buffer);
                    let mut response = Response::new(&mut tcp_stream);

                    if let Some((method, handler)) = self.path_handlers.get(request.path.as_str()) {
                        if request.method != *method {
                            response.send::<String>(
                                HttpStatus::MethodNotAllowed,
                                None
                            )
                        }

                        handler(request, response);
                    }
                }
            }

            return Ok(());
        } else {
            return Err("could't serve content".to_string());
        }
    }
}

#[derive(PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTION
}

pub struct Request {
    pub path: String,
    pub method: HttpMethod,
    pub headers: HashMap<String, String>,
    pub body: Option<String>
}

pub enum HttpStatus {
    Ok = 200,
    BadRequest = 400,
    UnAuthorised = 401,
    NotFound = 404,
    MethodNotAllowed = 405,
    InternalServerError = 500
}

pub struct Response<'a> {
    headers: HashMap<String, String>,
    tcp_stream: &'a mut TcpStream
}

impl<'a> Response<'a> {
    pub fn new(tcp_stream: &'a mut TcpStream) -> Self {
        Self { 
            headers: HashMap::new(),
            tcp_stream
        }
    }

    pub fn set_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }

    pub fn send<T>(
        &mut self,
        status: HttpStatus,
        body: Option<T>
    )
    where
        T: Display
    {
        self.tcp_stream.write_all(
            parser::parse_http_response(
                status,
                body.unwrap().to_string()
            ).as_bytes()
        ).unwrap();
    }
}