use poem::test::TestClient;

use crate::models::room::Room;

use super::{create::create_room_handler, payloads::RoomCreated};

#[tokio::test]
async fn create_room() {
    let route = create_room_handler();

    let cli = TestClient::new(route);

    let response = cli.post("/rooms").send().await;

    let _: RoomCreated = response.json().await.value().deserialize();
}

#[tokio::test]
async fn get_room() {
    let route = create_room_handler();

    let cli = TestClient::new(route);
    let response = cli.post("/rooms").send().await;
    let created_room: RoomCreated = response.json().await.value().deserialize();

    let path = format!("/rooms/{:?}", created_room.id.0);
    let response = cli.get(path).send().await;
    let _: Room = response.json().await.value().deserialize();
}
