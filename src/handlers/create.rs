/*
- [x] POST /rooms
- [] GET /rooms
- [] GET /rooms/:room_id
- [] GET /rooms/:room_id/messages

- [] POST /messages { room_id, message }
 */

use crate::models::room::RoomId;
use crate::{handlers::payloads::RoomCreated, models::room::Room};
use poem::http::StatusCode;
use poem::web::Json;
use poem::{handler, post, EndpointExt, Result, Route};
use poem::{Endpoint, Response};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref ROOMS: Arc<Mutex<HashMap<RoomId, Room>>> = Arc::new(Mutex::new(HashMap::new()));
}

#[handler]
fn create_room() -> Result<(StatusCode, Json<RoomCreated>)> {
    let new_room = Room::new();
    match add_room(new_room) {
        Ok(id) => Ok((StatusCode::CREATED, Json(RoomCreated { id }))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR.into()),
    }
}

fn add_room(room: Room) -> Result<RoomId, String> {
    if let Ok(mut rooms) = ROOMS.lock() {
        let id = room.id();
        rooms.insert(id, room);
        Ok(id)
    } else {
        Err("Unable to lock ROOMS".to_string())
    }
}

pub fn create_room_handler() -> impl Endpoint<Output = Response> {
    Route::new()
        .at("/rooms", post(create_room))
        .data(ROOMS.clone())
}
