use std::collections::HashMap;

use super::server::{
    HttpMethod,
    HttpStatus,
    Request
};

pub fn parse_http_request(request: String) -> Request {
    Request {
        path: String::from("/"),
        method: HttpMethod::GET,
        headers: HashMap::new(),
        body: None
    }
}

pub fn parse_http_response(
    status: HttpStatus,
    body: String
) -> String {
    "".to_string()
}