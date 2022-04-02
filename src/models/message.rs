use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::chatter::ChatterId;

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageId(Uuid);

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    id: MessageId,
    chatter_id: ChatterId,
    text: String,
}
