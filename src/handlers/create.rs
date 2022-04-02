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

use crate::{handlers::payloads::RoomCreated, models::room::Room};

#[handler]
pub fn create_room() -> Json<RoomCreated> {
    let new_room = Room::new();

    Json(RoomCreated {
        msg: "room created".to_string(),
        id: new_room.id(),
    })
}
