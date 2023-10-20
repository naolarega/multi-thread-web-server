use multi_thread_web_server::Server;

fn main() {
    let mut server = Server::new();

    server.add_router(
        "/hello",
        |req, res| res.respond(Some("Hello World"))
    );

    server.serve("0.0.0.0:8080");
}
