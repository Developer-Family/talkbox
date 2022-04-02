/*
- [x] POST /rooms
- [] GET /rooms
- [] GET /rooms/:room_id
- [] GET /rooms/:room_id/messages

- [] POST /messages { room_id, message }
 */

use poem::web::Json;
use poem::{handler, post, Route};

use crate::{handlers::payloads::RoomCreated, models::room::Room};

#[handler]
fn create_room() -> Json<RoomCreated> {
    let new_room = Room::new();

    Json(RoomCreated {
        msg: "room created".to_string(),
        id: new_room.id(),
    })
}

pub fn create_room_handler() -> Route {
    Route::new().at("/rooms", post(create_room))
}
