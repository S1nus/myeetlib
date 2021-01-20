use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    Identify,
    NotifyClient,
    DropClient,
    NewChatRequest,
    Auth,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IncomingMessage {
    pub message_type: MessageType,
    pub payload: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewChatRequest {
    pub nickname: String,
    pub uuid: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum QueueMessageType {
    Identify,
    PeerLocation,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueueOutgoingMessage {
    pub message_type: QueueMessageType,
    pub payload: String,
}
