use crate::config::middleware::auth_fn;
// use crate::controllers::auth::login::login_option;
use crate::controllers::auth::{login, me};
use actix_web::dev::Service;
use actix_web::web;
use futures_util::future::ok;



pub fn auth_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(web::resource("/login").route(web::post().to(login::login_controller)))
            .service(
                web::resource("/me")
                    .wrap_fn(|req, srv| {
                        let auth = auth_fn(req, "ADMIN");
                        match auth {
                            Ok(req) => return srv.call(req),
                            Err(err) => return {
                                let (req, _err) = err; 
                                let wrap = req.error_response(_err);
                                Box::pin(ok(wrap))
                            }
                        };
                    })
                    .route(web::get().to(me::index)),
            ),
    );
}
