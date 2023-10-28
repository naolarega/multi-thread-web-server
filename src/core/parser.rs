use std::{
    collections::HashMap,
    net::TcpStream
};

use super::server::{
    HttpMethod,
    HttpStatus,
    Request
};

pub fn parse_http_request(_: &TcpStream) -> Option<Request> {
    Some(
        Request {
            path: String::from("/"),
            method: HttpMethod::GET,
            headers: HashMap::new(),
            body: None
        }
    )
}

pub fn parse_http_response(
    status: HttpStatus,
    body: String
) -> String {
    let mut response = format!("HTTP/1.1 {}\n", status.to_string());

    response.push_str("Content-Type: text/plain\n");
    response.push_str(format!("Content-Length: {}\n\n", body.len()).as_str());
    response.push_str(body.as_str());

    response
}