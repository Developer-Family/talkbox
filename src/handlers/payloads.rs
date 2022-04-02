use serde::{Deserialize, Serialize};

use crate::models::room::RoomId;

#[derive(Serialize, Deserialize)]
pub struct RoomCreated {
    pub id: RoomId,
}
