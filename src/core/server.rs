use std::{collections::HashMap, net::TcpListener};

pub struct Server {
    path_handlers: HashMap<&'static str, Box<dyn Fn(String) -> String>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            path_handlers: HashMap::new(),
        }
    }

    pub fn add_router(
        &mut self,
        path: &'static str,
        handler: Box<dyn Fn(String) -> String>
    ) {
        self.path_handlers.insert(path, Box::new(handler));
    }

    pub fn serve(&mut self, host: &str) {
        let tcp_listener = TcpListener::bind(host);
    }
}
