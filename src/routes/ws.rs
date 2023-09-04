use actix::fut::ok;
use actix_web::{web, dev::Service};

use crate::controllers::ws;

pub fn ws_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/ws")
            .service(ws::send_message::index)
    );
}