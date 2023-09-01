mod config;
mod controllers;
mod interface;
mod messages;
pub mod models;
mod routes;
pub mod schema;

mod session;
mod socket_server;
pub mod types;
use crate::config::database::{async_connection, get_connection_pool};
use actix::{Actor, Addr};
use actix_cors::Cors;
use actix_web::{
    error,
    http::header::{self},
    web, App, Error, HttpRequest, HttpResponse, HttpServer,
};
use actix_web_actors::ws::{self};
use dotenv::dotenv;
use interface::global::ResponseJson;

async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<socket_server::ChatServer>>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    // let _srv = srv.clone();
    // println!("call chat route {:?}", )
    let room = path.into_inner();
    let ws = session::WsChatSession::new(room, srv.get_ref().clone());
    ws::start(ws, &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port_string = std::env::var("PORT")
        .expect("Port is undefined")
        .to_string();
    let port_int = port_string.parse::<u16>().unwrap();

    println!("application run on port : {} ", port_int);
    let server = socket_server::ChatServer::default().start();

    HttpServer::new(move || {
        let json_config = web::JsonConfig::default().limit(4096).error_handler(
            |err, _req: &actix_web::HttpRequest| {
                let error = ResponseJson {
                    message: err.to_string(),
                    status: String::from("fail"),
                    status_code: 400,
                };
                error::InternalError::from_response(err, HttpResponse::BadRequest().json(error))
                    .into()
            },
        );
        // Config cors
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .route("/{room}", web::get().to(chat_route))
            .app_data(web::Data::new(server.clone()))
            // .app_data(web::Data::from(app_state.clone()))
            .app_data(json_config)
            .app_data(web::Data::new(get_connection_pool().clone()))
            .app_data(web::Data::new(async_connection().clone()))
            .configure(routes::auth::auth_config)
            .configure(routes::users::users_config)
    })
    .bind("127.0.0.1:8000")
    .unwrap()
    .run()
    .await
}
