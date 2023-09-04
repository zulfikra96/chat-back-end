use actix::fut::ok;
use actix_web::{web, dev::Service};

use crate::{controllers::chat::{create_chat_room, get_chat_rooms}, config::middleware::auth_fn};

pub fn chat_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/chat")
            .wrap_fn(|req, srv| {
                let auth = auth_fn(req, "ADMIN|MEMBER");
                match auth {
                    Ok(req) => return srv.call(req),
                    Err(err) => {
                        return {
                            let (req, _err) = err;
                            let wrap = req.error_response(_err);
                            Box::pin(ok(wrap))
                        }
                    }
                };
            })
            .service(create_chat_room::index)
            .service(get_chat_rooms::index)
    );
}