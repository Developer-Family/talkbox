use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{chatter::ChatterId, message::MessageId};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RoomId(pub Uuid);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Room {
    id: RoomId,
    chatter_ids: Vec<ChatterId>,
    message_ids: Vec<MessageId>,
}

impl Room {
    pub fn new() -> Self {
        Room {
            id: RoomId(Uuid::new_v4()),
            chatter_ids: Vec::default(),
            message_ids: Vec::default(),
        }
    }

    pub fn id(&self) -> RoomId {
        self.id
    }
}
