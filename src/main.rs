mod core;

use core::server::Server;

fn main() {
    let mut server = Server::new();

    server.add_router(
        "/hello",
        Box::new(|data| data)
    );

    server.serve("0.0.0.0:8080");
}
