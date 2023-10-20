use multi_thread_web_server::{
    HttpMethod,
    Server
};

fn main() {
    let mut server = Server::new();

    server.add_router(
        HttpMethod::GET,
        "/hello",
        |_, res| res.respond(Some("Hello World"))
    );

    server.serve("0.0.0.0:8080").unwrap();
}
