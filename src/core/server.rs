use std::{
    collections::HashMap,
    net::TcpListener, fmt::Display
};

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
            return Ok(());
        } else {
            return Err("could't serve content".to_string());
        }
    }
}

pub enum HttpMethod {
    GET,
    POST,
    DELETE,
    PUT,
    PATCH,
    OPTION
}

pub struct Request {
    path: String,
    method: HttpMethod,
    headers: HashMap<String, String>,
    body: Option<String>
}

pub enum HttpStatus {
    Ok = 200,
    BadRequest = 400,
    UnAuthorised = 401,
    NotFound = 404,
    InternalServerError = 500
}

pub struct Response {
    status: HttpStatus,
    headers: HashMap<String, String>
}

impl Response {
    pub fn respond<T>(&self, body: Option<T>)
    where
        T: Display
    {
        /* Sends a response */
    }
}