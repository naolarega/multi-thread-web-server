use multi_thread_web_server::{
    HttpStatus,
    Server
};

fn main() {
    let mut server = Server::new();

    server.get("/", |_, mut res| {
        res.send(
            HttpStatus::Ok,
            Some("Hello World")
        );
    });

    server.serve("0.0.0.0:8080").unwrap();
}
