use crate::{
    config::middleware::auth_fn,
    controllers::users::{create_new_users, self},
};
use actix_web::{dev::Service, web};
use futures_util::future::ok;

pub fn users_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .wrap_fn(|req, srv| {
                let auth = auth_fn(req, "ADMIN");
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
            .route(web::post().to(create_new_users::index))
            .route(web::get().to(users::get_users::index))
    );
}
