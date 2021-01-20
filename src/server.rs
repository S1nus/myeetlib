use std::{
    sync::{
        Mutex, Arc,
    },
    collections::HashMap,
    net::SocketAddr,
    io::Error as IoError,
};

use async_tungstenite::tungstenite::protocol::Message;

use futures::prelude::*;
use futures::{
    future,
    channel::mpsc::{UnboundedSender, unbounded},
};

use async_std::{
    net::{TcpListener,
        TcpStream,
    },
    task,
};

use mysql::prelude::*;
use mysql::{Pool};

use linked_hash_map::LinkedHashMap;

use flurry::{HashMap as ConcurrentHashMap, HashSet as ConcurrentHashSet};

type Tx = UnboundedSender<Message>;

type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;
type UUIDMap = Arc<Mutex<HashMap<String, String>>>;
type RoutingTable = Arc<ConcurrentHashMap<String, String>>;
type ServerList = Arc<ConcurrentHashSet<String>>;
type ChatQueue = Arc<Mutex<LinkedHashMap<String, String>>>;

#[derive(Clone, Debug)]
pub struct Server {
    peermap: PeerMap,
    uuidmap: UUIDMap,
    // remove pub after debugging
    pub routing_table: RoutingTable,
    pub server_list: ServerList,
    pub chat_queue: ChatQueue,
    chat_size: usize,
    mysql_pool: Pool,
}

pub fn new_server(url: String) -> Server {
    Server {
        peermap: PeerMap::new(Mutex::new(HashMap::new())),
        uuidmap: UUIDMap::new(Mutex::new(HashMap::new())),
        routing_table: RoutingTable::new(ConcurrentHashMap::new()),
        server_list: ServerList::new(ConcurrentHashSet::new()),
        chat_queue: ChatQueue::new(Mutex::new(LinkedHashMap::new())),
        chat_size: 5,
        mysql_pool: Pool::new(url).unwrap(),
    }
}

impl Server {

    pub fn add_server(&self, uuid: String) {
        self.server_list.insert(String::from(&uuid), &self.server_list.guard());
    }

    pub fn remove_server(&self, uuid: String) {
        self.server_list.remove(&uuid, &self.server_list.guard());
    }

    pub fn notify_client(&self, server_id: String, client_id: String) {
        self.routing_table.insert(String::from(client_id), String::from(server_id), &self.routing_table.guard());
    }

    pub fn remove_client(&self, client_id: String) {
        self.routing_table.remove(&client_id, &self.routing_table.guard());
    }

    pub fn new_chat(&self, client_id: String, nickname: String) {
        self.chat_queue.clone().lock().unwrap()
            .insert(client_id, nickname);
        if (self.chat_queue.clone().lock().unwrap().len() >= self.chat_size) {
            println!("NEW CHAT");
            let mut conn = self.mysql_pool.get_conn().expect("Failed to get conn");
            let mut users : Vec<(String, String)> = Vec::new();
            for i in 0..5 {
                users.push(self.chat_queue.clone().lock().unwrap().pop_front().unwrap());
            }
            conn.exec_batch("INSERT INTO groupchatmemberships (user_id, groupchatid, nickname) VALUES (?, ?, ?)",
            users.iter().map(|entry| {
                vec![entry.0.to_string(), "somegroupchat".to_string(), entry.1.to_string()]
            })
            ).expect("insert failed.");
        }
        else {
            println!("NOt enough peeps for a new chat yet");
        }
    }

    pub fn cancel_chat(&self, client_id: String) {
        self.chat_queue.clone().lock().unwrap()
            .remove(&client_id);
    }
}
