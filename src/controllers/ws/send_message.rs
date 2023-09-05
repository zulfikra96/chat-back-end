use actix::{Addr, Response};
use actix_web::{get, HttpRequest, HttpResponse, web::{self, block}, ResponseError, http::{ self, header }};
use actix_web_actors::ws;
use deadpool_postgres::Pool;
use futures::executor::block_on;
use serde::{Deserialize, Serialize};
use derive_more::{ Display, Error };

use crate::{socket_server, session, config::{error, auth::JWT}, interface::global::ResponseJson};

#[derive(Debug, Deserialize)]
pub struct Header {
    pub token: Option<String>
}
#[derive(Debug, Deserialize, Serialize, Display, Error)]
#[display(fm = "{}", message)]
pub struct ErrorCustom {
    pub message: String
}

impl ResponseError for ErrorCustom {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::Unauthorized().json(ErrorCustom {
            message: self.message.clone()
        })
    }
}


#[get("/{room_id}/send-message")]
pub async fn index(
    req: HttpRequest,
    stream: web::Payload,
    path: web::Path<String>,
    srv: web::Data<Addr<socket_server::ChatServer>>,
    info: web::Query<Header>,
    db: web::Data<Pool>
) -> Result<HttpResponse, actix_web::Error>{
    let room = path.into_inner();
    let id;
    if let Some(token) = &info.token {
        let jwt = JWT {
            headers: req.headers(),
            token: &Some(token.to_string())
        };
        let claims = jwt.decode_token_directly();
        id = match claims {
            Ok(res) => res.claims.id,
            Err(err) => {
                println!("Erro : {:?}", err);
                return Err(ErrorCustom {
                    message:"Something went wrong".into()
                }.into());
            }
        };
    } else {
        return Err(ErrorCustom {
            message:"Something went wrong".into()
        }.into());
    }
    let pool = db.get().await.expect("Database is not set");
    let ws = session::WsChatSession::new(room, id,  srv.get_ref().clone(),pool );
    ws::start(ws, &req, stream)
}