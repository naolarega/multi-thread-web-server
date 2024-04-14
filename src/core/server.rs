use super::thread_pool::ThreadPool;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

type PathHandlerMap = HashMap<&'static str, HashMap<HttpMethod, fn(Request, Response)>>;

/// # Server
/// A server instance
pub struct Server {
    /// A map of route, method to handler
    path_handlers: PathHandlerMap,
    thread_pool: ThreadPool,
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}

impl Server {
    /// Create a new instance of a server
    pub fn new() -> Self {
        Self {
            path_handlers: HashMap::new(),
            thread_pool: ThreadPool::new(),
        }
    }

    /// Register a new get handler
    pub fn get(&mut self, path: &'static str, handler: fn(Request, Response)) {
        self.add_router(HttpMethod::GET, path, handler);
    }

    pub fn post(&mut self, path: &'static str, handler: fn(Request, Response)) {
        self.add_router(HttpMethod::POST, path, handler);
    }

    pub fn put(&mut self, path: &'static str, handler: fn(Request, Response)) {
        self.add_router(HttpMethod::PUT, path, handler);
    }

    pub fn patch(&mut self, path: &'static str, handler: fn(Request, Response)) {
        self.add_router(HttpMethod::PATCH, path, handler);
    }

    pub fn delete(&mut self, path: &'static str, handler: fn(Request, Response)) {
        self.add_router(HttpMethod::POST, path, handler);
    }

    pub fn option(&mut self, path: &'static str, handler: fn(Request, Response)) {
        self.add_router(HttpMethod::OPTION, path, handler);
    }

    fn add_router(
        &mut self,
        method: HttpMethod,
        path: &'static str,
        handler: fn(Request, Response),
    ) {
        if !path.starts_with("/") {
            panic!("path doesn't start with /");
        }

        if !self.path_handlers.contains_key(path) {
            self.path_handlers
                .insert(path, HashMap::from([(method, handler)]));
        } else {
            let methods_handler = self.path_handlers.get_mut(path).unwrap();

            if methods_handler.get(&method).is_some() {
                panic!("handler for method [{:?}] {path} already defined", method);
            }

            methods_handler.insert(method, handler);
        }
    }

    pub fn serve(&mut self, host: &str) -> Result<(), String> {
        if let Ok(tcp_listener) = TcpListener::bind(host) {
            for mut tcp_stream in tcp_listener.incoming().flatten() {
                let request = Request::from(&mut tcp_stream);
                let mut response = Response::new(tcp_stream);

                if let Some(method_handler) = self.path_handlers.get(request.path.as_str()) {
                    if let Some(handler) = method_handler.get(&request.method) {
                        let handler = *handler;

                        self.thread_pool.submit(move || handler(request, response));
                    } else {
                        response.set_status_code(HttpStatus::MethodNotAllowed);
                        response
                            .send(Some("Method not allowed".to_string()))
                            .unwrap();
                    }
                } else {
                    response.set_status_code(HttpStatus::NotFound);
                    response.send(Some("Not Found".to_string())).unwrap();
                }
            }

            Ok(())
        } else {
            Err("could't serve content".to_string())
        }
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTION,
}

impl From<&str> for HttpMethod {
    fn from(value: &str) -> Self {
        use HttpMethod::*;

        match value {
            "GET" => GET,
            "POST" => POST,
            "PUT" => PUT,
            "PATCH" => PATCH,
            "DELETE" => DELETE,
            "OPTION" => OPTION,
            _ => panic!("unssuported method"),
        }
    }
}

pub struct Request {
    pub path: String,
    pub method: HttpMethod,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl From<&mut TcpStream> for Request {
    fn from(stream: &mut TcpStream) -> Self {
        let mut stream_buffer_reader = BufReader::new(stream);
        let mut start_line_string = String::new();

        stream_buffer_reader
            .read_line(&mut start_line_string)
            .unwrap();

        let start_line = start_line_string
            .split(' ')
            .map(|a| a.to_string())
            .collect::<Vec<String>>();
        let mut headers = HashMap::new();
        let mut content_length = 0;

        loop {
            let mut header = String::new();

            stream_buffer_reader.read_line(&mut header).unwrap();

            if &header == "\r\n" {
                break;
            }

            let key_value = header
                .split(':')
                .map(|a| a.to_string())
                .collect::<Vec<String>>();

            if key_value[0].to_lowercase() == "content-length" {
                content_length = key_value[1].trim().parse().unwrap();
            }

            if key_value.len() >= 2 {
                headers.insert(
                    key_value[0].to_lowercase(),
                    key_value[1..].join(":").trim().to_owned(),
                );
            }
        }

        let mut body_buf = vec![u8::default(); content_length];
        let mut body = None;

        if content_length > 0 {
            stream_buffer_reader.read_exact(&mut body_buf).unwrap();
            body = Some(String::from_utf8_lossy(&body_buf).to_string());
        }

        Self {
            method: HttpMethod::from(start_line[0].as_str()),
            path: start_line[1].to_owned(),
            headers,
            body,
        }
    }
}

pub enum HttpStatus {
    Ok = 200,
    BadRequest = 400,
    UnAuthorised = 401,
    NotFound = 404,
    MethodNotAllowed = 405,
    InternalServerError = 500,
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
            InternalServerError => String::from("500 Internal Server Error"),
        }
    }
}

pub struct Response {
    tcp_stream: TcpStream,
    status_code: Option<HttpStatus>,
    headers: HashMap<String, String>,
}

impl Response {
    pub fn new(tcp_stream: TcpStream) -> Self {
        Self {
            tcp_stream,
            status_code: None,
            headers: HashMap::new(),
        }
    }

    pub fn set_status_code(&mut self, status_code: HttpStatus) -> &mut Self {
        self.status_code = Some(status_code);
        self
    }

    pub fn set_header(&mut self, key: &str, value: &str) -> &mut Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn send<T>(&mut self, body: Option<T>) -> Result<bool, &str>
    where
        T: ToString,
    {
        if let Some(status_code) = &self.status_code {
            self.tcp_stream
                .write(format!("HTTP/1.1 {}\r\n", status_code.to_string()).as_bytes())
                .unwrap();
        } else {
            return Err("status code not set");
        }

        for (key, value) in self.headers.iter() {
            self.tcp_stream
                .write(format!("{}:{}\r\n", key.to_lowercase(), value).as_bytes())
                .unwrap();
        }

        if let Some(body) = body {
            let body = body.to_string();

            self.tcp_stream
                .write(format!("content-length:{}\r\n", body.len()).as_bytes())
                .unwrap();
            self.tcp_stream.write(b"\r\n").unwrap();

            self.tcp_stream.write(body.as_bytes()).unwrap();
        } else {
            self.tcp_stream.write(b"\r\n").unwrap();
        }

        self.tcp_stream.flush().unwrap();

        Ok(true)
    }
}
