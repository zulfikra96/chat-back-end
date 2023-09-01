use crate::{interface::global::ResponseJsonWithData};
use actix_web::{web, HttpResponse};
use deadpool_postgres::Pool;
use serde::Serialize;
use uuid;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize)]
struct Users {
    id: uuid::Uuid,
    nrp: String,
    name: String,
    created_at: NaiveDateTime,
}



#[allow(dead_code)]
pub async fn index(db: web::Data<Pool>) -> HttpResponse {
    let client = db.get().await.unwrap();
    let smt = client
        .prepare_cached("SELECT id, nrp, name, created_at FROM users ")
        .await
        .unwrap();
    let rows = client.query(&smt, &[]).await.unwrap();

    let results: Vec<Users> = rows.iter().map(|res| Users { id: res.get(0), nrp: res.get(1), name: res.get(2), created_at: res.get(3) }).collect();

    HttpResponse::Ok().json(ResponseJsonWithData{
        data: results,
        message:"success get data".to_string(),
        status:"success".to_string(),
        status_code:200
    })
}
