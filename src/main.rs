use talkbox::start_server;

#[tokio::main]
async fn main() {
    let server = start_server().await;

    match server {
        Ok(_) => println!("Server Started"),
        Err(_) => panic!(),
    }
}
