/*
POST /rooms
GET /rooms
GET /rooms/:room_id
GET /rooms/:room_id/messages

POST /messages { room_id, message }

/hello/toto
 */

use poem::handler;
use poem::web::Json;
use serde::{Deserialize, Serialize};

use crate::models::room::{Room, RoomId};

#[derive(Serialize, Deserialize)]
pub struct RoomCreated {
    pub msg: String,
    pub id: RoomId,
}

#[handler]
pub fn create_room() -> Json<RoomCreated> {
    let new_room = Room::new();

    Json(RoomCreated {
        msg: "room created".to_string(),
        id: new_room.id(),
    })
}

#[cfg(test)]
mod tests {
    use poem::{post, test::TestClient, Route};

    use super::{create_room, RoomCreated};

    #[tokio::test]
    async fn hello_test() {
        let route = Route::new().at("/rooms", post(create_room));

        let cli = TestClient::new(route);

        let response = cli.post("/rooms").send().await;

        let _: RoomCreated = response.json().await.value().deserialize();
    }
}
