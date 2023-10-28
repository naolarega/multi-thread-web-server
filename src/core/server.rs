use std::{
    collections::HashMap,
    fmt::Display,
    io::Write,
    net::{TcpListener, TcpStream}
};

use super::parser;

pub struct Server {
    path_handlers: HashMap<&'static str, HashMap<HttpMethod, fn(Request, Response)>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            path_handlers: HashMap::new(),
        }
    }

    pub fn get(
        &mut self,
        path: &'static str,
        handler: fn(Request, Response)
    ) {
        self.add_router(
            HttpMethod::GET,
            path,
            handler
        );
    }

    pub fn post(
        &mut self,
        path: &'static str,
        handler: fn(Request, Response)
    ) {
        self.add_router(
            HttpMethod::POST,
            path,
            handler
        );
    }

    pub fn put(
        &mut self,
        path: &'static str,
        handler: fn(Request, Response)
    ) {
        self.add_router(
            HttpMethod::PUT,
            path,
            handler
        );
    }

    pub fn patch(
        &mut self,
        path: &'static str,
        handler: fn(Request, Response)
    ) {
        self.add_router(
            HttpMethod::PATCH,
            path,
            handler
        );
    }

    pub fn delete(
        &mut self,
        path: &'static str,
        handler: fn(Request, Response)
    ) {
        self.add_router(
            HttpMethod::POST,
            path,
            handler
        );
    }

    pub fn option(
        &mut self,
        path: &'static str,
        handler: fn(Request, Response)
    ) {
        self.add_router(
            HttpMethod::OPTION,
            path,
            handler
        );
    }

    fn add_router(
        &mut self,
        method: HttpMethod,
        path: &'static str,
        handler: fn(Request, Response)
    ) {
        if !self.path_handlers.contains_key(path) {
            self.path_handlers.insert(
                path,
                HashMap::from([
                    (method, handler)
                ])
            );
        } else {
            let methods_handler = self.path_handlers.get_mut(path).unwrap();

            if methods_handler.get(&method).is_some() {
                panic!("handler for method [{:?}] : {path} already defined", method);
            }

            methods_handler.insert(method, handler);
        }
    }

    pub fn serve(&mut self, host: &str) -> Result<(), String> {
        if let Ok(tcp_listener) = TcpListener::bind(host) {
            for tcp_stream in tcp_listener.incoming() {
                if let Ok(mut tcp_stream) = tcp_stream {
                    if let Some(request) = parser::parse_http_request(&tcp_stream) {
                        let mut response = Response::new(&mut tcp_stream);
    
                        if let Some(method_handler) = self.path_handlers.get(request.path.as_str()) {
                            if let Some(handler) = method_handler.get(&request.method) {
                                handler(request, response);
                            } else {
                                response.send(
                                    HttpStatus::MethodNotAllowed,
                                    Some("Method not allowed".to_string())
                                );
                            }
                        } else {
                            response.send(
                                HttpStatus::NotFound,
                                Some("Not Found".to_string())
                            );
                        }
                    }
                }
            }

            return Ok(());
        } else {
            return Err("could't serve content".to_string());
        }
    }
}

#[derive(
    Eq,
    PartialEq,
    Hash,
    Debug
)]
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

impl ToString for HttpStatus {
    fn to_string(&self) -> String {
        use HttpStatus::*;
        
        match self {
            Ok => String::from("200 OK"),
            BadRequest => String::from("400 Bad Request"),
            UnAuthorised => String::from("401 Unauthorized"),
            NotFound => String::from("404 Not Found"),
            MethodNotAllowed => String::from("405 Method Not Allowed"),
            InternalServerError => String::from("500 Internal Server Error")
        }
    }
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