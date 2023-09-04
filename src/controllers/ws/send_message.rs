use actix::Addr;
use actix_web::{get, HttpRequest, HttpResponse, web};
use actix_web_actors::ws;

use crate::{socket_server, session};


#[get("/{room_id}/send-message")]
pub async fn index(
    req: HttpRequest,
    stream: web::Payload,
    path: web::Path<String>,
    srv: web::Data<Addr<socket_server::ChatServer>>
) -> Result<HttpResponse, actix_web::Error>{
    let room = path.into_inner();
    let ws = session::WsChatSession::new(room, srv.get_ref().clone());
    ws::start(ws, &req, stream)
}