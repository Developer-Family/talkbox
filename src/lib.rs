use poem::{listener::TcpListener, Server};

use crate::handlers::create::create_room_handler;

mod handlers;
mod models;

pub async fn start_server() -> Result<(), std::io::Error> {
    let app = create_room_handler();

    println!("Starting server on address 127.0.0.1:3000");

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}
