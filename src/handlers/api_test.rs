use poem::test::TestClient;

use super::{create::create_room_handler, payloads::RoomCreated};

#[tokio::test]
async fn hello_test() {
    let route = create_room_handler();

    let cli = TestClient::new(route);

    let response = cli.post("/rooms").send().await;

    let _: RoomCreated = response.json().await.value().deserialize();
}
