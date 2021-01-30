use serde::{Deserialize, Serialize};

/*
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
*/

/*
 *
 * This enum defines messages that servers send to the Queue
 **/
#[derive(Serialize, Deserialize, Debug)]
pub enum QueueMessage {
    /*
     * Server sends its UUID to the Queue when it spins up
     * The Queue should log this in its Server List hashset
     * and remove it when the connection to this server closes
     */
    Identify {
        server_id: String
    },

    /*
     * Server sends a client ID to the queue, and the queue saves the <client_id, server_id> mapping in its routing table.
     */
    NotifyClient {
        client_id: String,
    },

    /*
     * Server sends this to the Queue when a client disconnects (or gets kicked).
     * Queue should remove it's mapping from the routing table.
     */

    DropClient {
        client_id: String,
    },

    /*
     * Queue should add the <client_id, client_nick> mapping to the LinkedHashMap representing the
     * new chat queue.
     * If the new chat queue length is 5, it should remove those five, create a chat in the
     * database, and notify the clients.
     */

    NewChatRequest {
        client_id: String,
        client_nick: String,
    },

    /* 
     * pull the client_id out of the new chat queue.
     */

    CancelChatRequest {
        client_id: String,
    },

    /* 
     * Queue adds the chat message to the database, then routes the message along to its intended
     * recipients.
     */
    ChatMessage {
        client_id: String,
        chat_id: String,
        message_text: String,
    },
}

pub enum ClientMessage {
    Auth {
        client_id: String,
    },
    NewChatRequest {
        client_id: String,
        client_nick: String,
    },
    CancelChatRequest {
        client_id: String,
    },
    ChatMessage {
        client_id: String,
        chat_id: String,
        message_text: String,
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    RoutedChatMessage {
        client_id: String,
        chat_id: String,
        message_text: String,
        nick: String,
    },
    NewChatCreated {
        client_id: String,
        chat_id: String,
    },
}
