use poem::{listener::TcpListener, post, Route, Server};

use crate::handlers::create_room;

mod handlers;
mod models;

pub async fn start_server() -> Result<(), std::io::Error> {
    let app = Route::new().at("/room", post(create_room));

    println!("Starting server on address 127.0.0.1:3000");

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}
