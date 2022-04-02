use poem::{post, test::TestClient, Route};

use super::{create::create_room, payloads::RoomCreated};

#[tokio::test]
async fn hello_test() {
    let route = Route::new().at("/rooms", post(create_room));

    let cli = TestClient::new(route);

    let response = cli.post("/rooms").send().await;

    let _: RoomCreated = response.json().await.value().deserialize();
}
