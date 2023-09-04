use actix_web::{get, web, HttpResponse};
use deadpool_postgres::Pool as TokioPool;
use serde::{Deserialize, Serialize};

use crate::{controllers::auth::login::ErrorResponse, interface::global::ResponseJsonWithData};

#[derive(Deserialize)]
pub struct Info {
    pub search: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Users {
    id: uuid::Uuid,
    name: String,
}

#[get("/users")]
pub async fn index(info: web::Query<Info>, db: web::Data<TokioPool>) -> HttpResponse {
    let conn = db.get().await.expect("Something went wrong with DB");
    let  smt;
    let mut exec;
    if let Some(search) = &info.search {
        smt = conn.prepare_cached("SELECT id, name FROM users WHERE name ILIKE $1").await;
        exec = conn.query(&smt.unwrap(), &[&format!("%{}%", &search.to_string())]).await
    } else {
        smt = conn.prepare("SELECT id, name FROM users LIMIT 10").await;
        exec = conn.query(&smt.unwrap(), &[]).await;
    }

    exec = match exec {
        Ok(res) => Ok(res),
        Err(err) => {
            println!("Error : {:?}", err);
            return HttpResponse::BadGateway().json(ErrorResponse {
                message: format!("Something went wrong {err}"),
                status: "error".to_string(),
                status_code: 400,
            });
        }
    };

    let rows: Vec<Users> = exec.unwrap()
        .iter()
        .map(|row| Users {
            id: row.get(0),
            name: row.get(1),
        })
        .collect();
    HttpResponse::Ok().json(ResponseJsonWithData {
        data: rows,
        message: "success".to_string(),
        status: "success".to_string(),
        status_code: 200,
    })
}
