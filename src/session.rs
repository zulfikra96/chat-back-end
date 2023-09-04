use actix::prelude::*;
use actix_web_actors::ws;
use std::time::{Duration, Instant};

use crate::{socket_server, messages};

// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug)]
pub struct WsChatSession {
    pub id: uuid::Uuid,
    pub hb: Instant,
    pub room: String,
    pub name: Option<String>,
    pub addr: Addr<socket_server::ChatServer>,
}


impl WsChatSession {
    #[allow(dead_code)]
    pub fn new(room: String, addr: Addr<socket_server::ChatServer>) -> WsChatSession {
        WsChatSession {
            addr,
            hb:Instant::now(),
            id:uuid::Uuid::new_v4(),
            name:None,
            room
        }
    }
}



impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        // println!("call started {:?}", self.counter);
        let addr = ctx.address();
        
        self.addr
            .send(messages::Connect {
                addr: addr.recipient(),
                self_id: self.id,
                room_id: self.room.to_string()
            })
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(_res) => (),
                    Err(_) => {
                        ctx.stop()
                    },
                }

                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.addr.do_send(messages::Disconnect { id: self.id });

        Running::Stop
    }
}

impl WsChatSession {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("websocket client hearbeat failed, disconnecting  ");

                // act.addr.do_send(socket_server::Disconnect { id: act.id });

                ctx.stop();

                return;
            }

            ctx.ping(b"PING");
        });
    }
}

impl Handler<messages::Message> for WsChatSession {
    type Result = ();
    fn handle(&mut self, msg: messages::Message, ctx: &mut Self::Context)  {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // println!("Call stream handler");
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };


        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                println!("trace message {:?}", self.addr);
                self.addr.do_send(messages::ClientMessage{
                    msg:text.to_string(),
                    id: self.id,
                    room: self.room.to_owned()
                })
            }
            ws::Message::Binary(_) => println!("Unexpected binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}
