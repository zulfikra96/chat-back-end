use crate::{config::auth::JWT, interface::global::{ResponseJson, ResponseJsonWithData}};
use actix_web::{post, web, HttpRequest, HttpResponse};
use deadpool_postgres::{Object, Pool};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomBody {
    pub to_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseData {
    pub id: uuid::Uuid
}

async fn validator(data: &web::Json<RoomBody>, db: &Object, user_id: &uuid::Uuid) -> bool {
    let to_id = &data.to_id;
    if to_id.is_empty() {
        return false;
    }

    // check is from id exists in db
    let to_id_smt = db
        .prepare_cached("SELECT 1 FROM users WHERE id = $1")
        .await
        .expect("Something occurs here");

    let id = match uuid::Uuid::parse_str(&data.to_id) {
        Ok(_id) => _id,
        Err(err) => {
            println!("Error uuid : {}", err);
            return false;
        }
    };

    let query = r#"
        SELECT 1 FROM rooms 
        INNER JOIN users_room user_1 ON rooms.id = user_1.room_id
        INNER JOIN users_room user_2 ON rooms.id = user_2.room_id 
        WHERE user_1.user_id = $1 AND user_2.user_id = $2
    "#;
    let is_exist_user_room = db.prepare_cached(query).await.unwrap();
    let is_exists_user_exec = db
        .query(
            &is_exist_user_room,
            &[&uuid::Uuid::parse_str(&to_id).unwrap(), &user_id],
        )
        .await
        .unwrap();
    if is_exists_user_exec.len() != 0 {
        return false;
    }

    let query = db
        .query(&to_id_smt, &[&id])
        .await
        .expect("Somthing occurs when query");
    if query.len() == 0 {
        return false;
    }
    true
}

#[post("/rooms/create")]
pub async fn index(
    req: HttpRequest,
    body: web::Json<RoomBody>,
    db: web::Data<Pool>,
) -> HttpResponse {
    let pool = db.get().await.expect("Connection database must be set");
    let jwt = JWT {
        headers: req.headers(),
        token: &None
    };
    let decode_jwt = jwt.decode_token().unwrap();
    let claims = decode_jwt.claims;
    // Validator
    if !validator(&body, &pool, &claims.id).await {
        return HttpResponse::BadRequest().json(ResponseJson {
            message: "Invalid data".to_string(),
            status: "fail".to_string(),
            status_code: 400,
        });
    }

    let user_id = claims.id.to_string();
    drop(claims);
    let to_id = &body.to_id;
    let rooms_name = format!("{user_id}_{to_id}");
    let smt = pool
        .prepare("INSERT INTO rooms (name, is_group) VALUES($1, $2) RETURNING id")
        .await
        .unwrap();
    let sql = pool.query_one(&smt, &[&rooms_name, &false]).await;

    match sql {
        Ok(_) => {}
        Err(err) => {
            println!("Error sql: {}", err);
            return HttpResponse::BadRequest().json(ResponseJson {
                message: "Something went wrong".to_string(),
                status: "fail".to_string(),
                status_code: 400,
            });
        }
    };
    let row = sql.unwrap();
    let room_id = row.get::<&str, uuid::Uuid>("id");

    let smt = pool
        .prepare("INSERT INTO users_room (room_id, user_id) VALUES($1, $2)")
        .await
        .unwrap();
    pool.execute(&smt, &[&room_id, &uuid::Uuid::parse_str(&to_id).unwrap()])
        .await
        .unwrap();
    pool.execute(&smt, &[&room_id, &uuid::Uuid::parse_str(&user_id).unwrap()])
        .await
        .unwrap();

    HttpResponse::Ok().json(ResponseJsonWithData {
        message: "success to create room".to_string(),
        status: "success".to_string(),
        status_code: 200,
        data: ResponseData {
            id: room_id
        }
    })
}
