use std::str::FromStr;

use actix_web::{get, web, HttpRequest, HttpResponse};
use chrono::{NaiveDate, NaiveDateTime};
use deadpool_postgres::{Object, Pool};
use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};

use crate::{config::auth::JWT, interface::global::{ResponseJson, ResponseJsonWithData}};

#[derive(Debug, Deserialize, Serialize, ToSql, FromSql)]
pub struct ChatList {
    id: uuid::Uuid,
    user_id: uuid::Uuid,
    message: String,
    name: String,
    created_at: NaiveDateTime,
}

async fn authorized_room(db: &Object, room_id: &uuid::Uuid, user_id: &uuid::Uuid) -> bool {
    let smt = db
        .prepare_cached(
            r#"
        SELECT 1 
        FROM users_room
        WHERE room_id = $1 AND user_id = $2
    "#,
        )
        .await
        .map_err(|e| {
            println!("Something went wrong {:?}", e);
        });
    let result = db
        .query(&smt.unwrap(), &[room_id, user_id])
        .await
        .map_err(|e| {
            println!("Something went wrong {:?} ", e);
        });
    if result.iter().len() == 0 {
        return false;
    }
    true
}

#[get("/rooms/{id}")]
pub async fn index(req: HttpRequest, db: web::Data<Pool>, id: web::Path<String>) -> HttpResponse {
    let jwt = JWT {
        token: &None,
        headers: req.headers(),
    };
    let response_error = ResponseJson {
        message: "Something went wrong".to_string(),
        status: "fail".to_string(),
        status_code: 422,
    };
    let decode_token = match jwt.decode_token() {
        Ok(res) => res,
        Err(err) => {
            println!("Error: {:?}", err);
            return HttpResponse::MethodNotAllowed().json(&response_error);
        }
    };

    let pool = match db.get().await {
        Ok(res) => res,
        Err(err) => {
            println!("Error : {}", err);
            return HttpResponse::MethodNotAllowed().json(&response_error);
        }
    };
    let _id = match uuid::Uuid::from_str(&id) {
        Ok(res) => res,
        Err(err) => {
            println!("Err : {:?}", err);
            return HttpResponse::MethodNotAllowed().json(&response_error);
        }
    };
    let is_authorized_room = authorized_room(&pool, &_id, &decode_token.claims.id).await;
    if !is_authorized_room {
        return HttpResponse::MethodNotAllowed().json(&response_error);
    }
    let query = r#"
        SELECT rooms_message.id, rooms_message.user_id, message, rooms_message.created_at, users.name
        FROM rooms_message
        INNER JOIN users ON rooms_message.user_id = users.id
        WHERE rooms_message.room_id = $1
        ORDER BY created_at ASC
    "#;
    let smt = match pool.prepare_cached(&query).await {
        Ok(res) => res,
        Err(err) => {
            println!("err :{:?}" ,err);
            return HttpResponse::MethodNotAllowed().json(&response_error);

        }
    };
    let result: Vec<ChatList> = match pool.query(&smt, &[&_id]).await {
        Ok(res) => res
            .iter()
            .map(|row| ChatList {
                id: row.get(0),
                user_id: row.get(1),
                message: row.get(2),
                created_at: row.get(3),
                name: row.get(4),
            })
            .collect(),
        Err(err) => {
            println!("Error {:?} ", err);
            return HttpResponse::MethodNotAllowed().json(&response_error);
        }
    };

    HttpResponse::Ok().json(ResponseJsonWithData {
        data: result,
        message:"success".to_string(),
        status:"success".to_string(),
        status_code: 200
    })
}
