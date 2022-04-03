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
use poem::web::{Json, Path};
use poem::{get, handler, post, EndpointExt, Result, Route};
use poem::{Endpoint, Response};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

lazy_static! {
    static ref ROOMS: Arc<Mutex<HashMap<RoomId, Room>>> = Arc::new(Mutex::new(HashMap::new()));
}

#[handler]
fn open_room() -> Result<(StatusCode, Json<RoomCreated>)> {
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

#[handler]
fn get_room(Path(room_id): Path<Uuid>) -> Result<(StatusCode, Json<Room>)> {
    match retrieve_room(RoomId(room_id)) {
        Ok(room) => Ok((StatusCode::OK, Json(room))),
        Err(_) => Err(StatusCode::NOT_FOUND.into()),
    }
}

fn retrieve_room(room_id: RoomId) -> Result<Room, String> {
    if let Ok(rooms) = ROOMS.lock() {
        rooms
            .get(&room_id)
            .cloned()
            .ok_or_else(|| "Unable to retrieve room".to_string())
    } else {
        Err("Unable to lock Rooms".to_string())
    }
}

pub fn create_room_handler() -> impl Endpoint<Output = Response> {
    Route::new()
        .at("/rooms", post(open_room))
        .at("/rooms/:room_id", get(get_room))
        .data(ROOMS.clone())
}
