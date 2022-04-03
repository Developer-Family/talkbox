use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct ChatterId(Uuid);

#[derive(Debug, Serialize, Deserialize)]
pub struct Chatter {
    id: ChatterId,
}
