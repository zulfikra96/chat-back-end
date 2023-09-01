use actix_web::{web, dev::Service};
use futures_util::future::ok;

use crate::{controllers::members, config::middleware::auth_fn};

#[allow(dead_code)]
pub fn members_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/members")
            .wrap_fn(|req, srv| {
                let auth = auth_fn(req, "MEMBER|ADMIN");
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
            .service(members::get_users::index),
    );
}
