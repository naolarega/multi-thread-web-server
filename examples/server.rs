use multi_thread_web_server::{HttpStatus, Server};

fn main() {
    let mut server = Server::new();

    server.get("/", |_, mut res| {
        res.set_status_code(HttpStatus::Ok);
        res.send(Some("Hello World")).unwrap();
    });

    server.serve("0.0.0.0:8080").unwrap();
}
