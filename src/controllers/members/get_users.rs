use actix_web::{get, HttpResponse};

#[get("/users")]
pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body("")
}