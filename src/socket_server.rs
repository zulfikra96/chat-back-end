use std::{collections::{HashMap, HashSet}, str::FromStr};

use actix::prelude::*;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use postgres_types::ToSql;
use serde::{Deserialize, Serialize};

use crate::messages::{self, *};

pub struct ListRooms;

impl actix::Message for ListRooms {
    type Result = Vec<String>;
}

#[derive(Debug, Clone)]
pub struct ChatServer {
    sessions: HashMap<uuid::Uuid, Recipient<messages::Message>>,
    rooms: HashMap<String, HashSet<uuid::Uuid>>,
}

impl Default for ChatServer {
    fn default() -> ChatServer {
        // println!("Call new");
        ChatServer {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
        }
    }
}
impl ChatServer {
    pub fn send_message(&self, to_id: uuid::Uuid, message: &str) {
        if let Some(addr) = self.sessions.get(&to_id) {
            addr.do_send(messages::Message(message.to_owned()))
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) {
        println!("Someone disconnect");

        let mut rooms: Vec<String> = Vec::new();

        if self.sessions.remove(&msg.id).is_some() {
            for (name, sessions) in &mut self.rooms {
                if sessions.remove(&msg.id) {
                    rooms.push(name.to_owned());
                }
            }
        }
    }
}

impl Handler<Connect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: messages::Connect, _: &mut Self::Context) -> Self::Result {
        println!("Someone joined");
        // println!("{:?}", std::thread::current());
        // println!("joined id {}", id);
        // println!("room {:?}", msg.addr);
        // broadcast to main room that someone joined
        // let mut response_message = ResponseMessage {
        //     message: "someone joined".to_string(),
        //     message_type: "message".to_string(),
        //     data: None,
        // };
        // let mut message = serde_json::to_string(&response_message).unwrap();
        // self.send_message("main", "Someone joined", 0);
        // println!(" trace {:?}", msg);

        self.rooms
            .entry(msg.room_id.clone())
            .or_insert_with(HashSet::new)
            .insert(msg.self_id);

        self.rooms
            .get(&msg.room_id)
            .unwrap()
            .iter()
            .filter(|conn_id| *conn_id.to_owned() != msg.self_id)
            .for_each(|conn_id| {
                self.send_message(*conn_id, &format!("{} just joined!", msg.self_id))
            });

        self.sessions.insert(msg.self_id, msg.addr);
        // println!("session {:?} ", self.sessions);

        // println!("rooms list {:?}", self.rooms);

        // response_message = ResponseMessage {
        //     message: format!("total visitor {count}"),
        //     message_type: "message".to_string(),
        //     data: None,
        // };
        // message = serde_json::to_string(&response_message).unwrap();
        // self.send_message("main", &format!("Total visitor count "), 0);
    }
}

impl Handler<ListRooms> for ChatServer {
    type Result = MessageResult<ListRooms>;
    fn handle(&mut self, _: ListRooms, _: &mut Self::Context) -> Self::Result {
        let mut rooms: Vec<String> = Vec::new();
        // println!("call list room");
        for key in self.rooms.keys() {
            rooms.push(key.to_owned());
        }

        MessageResult(rooms)
    }
}

#[derive(Debug, Deserialize, Serialize, ToSql)]
struct MessageJson {
    token: String,
    message: String,
    created_at: i64
}

impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, ctx: &mut Context<Self>) {
        let room = msg.room.clone();
        let id = msg.id.clone();
        let  message = serde_json::from_str::<MessageJson>(&msg.msg.to_string()).unwrap();
        println!("Message {}", message.created_at);
        let created_at = match  NaiveDateTime::from_timestamp_millis(message.created_at) {
            Some(res) => res,
            _ => return println!("return Nothing")
        };

        
        let fut = async move {
            let smt = msg.db.prepare("INSERT INTO rooms_message (room_id, user_id, message, created_at) VALUES($1, $2, $3, $4)").await.unwrap();
            // println!("trace {:?}", message);
            msg.db.execute(&smt, &[&uuid::Uuid::parse_str(&room).unwrap(),&id, &message.message.to_string(), &created_at]).await.unwrap();
        };
        let fut = actix::fut::wrap_future::<_, Self>(fut);
        ctx.spawn(fut);
        self.rooms
            .get(&msg.room)
            .unwrap()
            .iter()
            .for_each(|client| {
                self.send_message(*client, msg.msg.as_str());
            })

        
    }
}

// impl Handler<Join> for ChatServer {
//     type Result = ();
//     fn handle(&mut self, msg: Join, ctx: &mut Self::Context) -> Self::Result {
//         let Join { id, name } = msg;
//         let mut rooms = Vec::new();
//         for (n, sessions) in &mut self.rooms {
//             if sessions.remove(&id) {
//                 rooms.push(n.to_owned());
//             }
//         }

//         let mut response_message = ResponseMessage {
//             message: "someone disconnect".to_string(),
//             message_type: "message".to_string(),
//             data: None,
//         };

//         let mut message = serde_json::to_string(&response_message).unwrap();

//         for room in rooms {
//             self.send_message(&room, "Someone disconnected", 0);
//         }

//         response_message = ResponseMessage {
//             message: "someone connected".to_string(),
//             message_type: "message".to_string(),
//             data: None,
//         };

//         message = serde_json::to_string(&response_message).unwrap();

//         self.rooms.entry(name.clone()).or_default().insert(id);

//         self.send_message(&name, "Someone connected", 0)
//     }
// }
