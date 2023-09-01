use actix::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(Message, Debug, Clone)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<Message>,
    pub self_id: uuid::Uuid,
    pub room_id: String

}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: uuid::Uuid,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub id: uuid::Uuid,
    pub msg: String,
    pub room: String,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseMessageData {
    // to rooms
    pub to: String,
    // users
    pub from: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseMessage {
    pub message: String,
    pub message_type: String,
    pub data: Option<ResponseMessageData>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    pub id: usize,
    pub name: String,
}
