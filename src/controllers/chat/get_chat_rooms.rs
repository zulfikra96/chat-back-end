use actix_web::{get, web::Data, HttpRequest, HttpResponse};
use deadpool_postgres::Pool;
use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::{
    config::auth::JWT,
    interface::global::{ResponseJson, ResponseJsonWithData},
};

#[derive(Deserialize, Serialize, Debug, ToSql, FromSql)]
struct Rooms {
    id: uuid::Uuid,
    name: String,
    is_group: bool,
    users: Vec<Value>,
}

#[get("/rooms")]
pub async fn index(req: HttpRequest, db: Data<Pool>) -> HttpResponse {
    let pool = db.get().await.expect("Db must be set");
    let decode = JWT {
        headers: req.headers(),
    };
    let token_data = decode.decode_token().unwrap();
    let query = r#"
    SELECT 
        rooms.id, 
        rooms.name, 
        is_group,
        (
            SELECT ARRAY_AGG(row_to_json(usr)) FROM (
                SELECT user_id, users.name
                
                FROM users_room 
                INNER JOIN users ON users_room.user_id = users.id
                WHERE users_room.room_id = rooms.id
            ) as usr
        ) as users
    FROM rooms
    INNER JOIN users_room ON rooms.id = users_room.room_id  
    WHERE users_room.user_id = $1
    "#;

    let smt = pool.prepare_cached(&query).await.unwrap();
    let fetch = pool.query(&smt, &[&token_data.claims.id]).await;

    match &fetch {
        Ok(res) => res,
        Err(err) => {
            println!("Error : {:?}", err);
            return HttpResponse::BadRequest().json(ResponseJson {
                message: "something went wrong".to_string(),
                status: "fail".to_string(),
                status_code: 400,
            });
        }
    };
    let rooms: Vec<Rooms> = fetch
        .unwrap()
        .iter()
        .map(|row| Rooms {
            id: row.get(0),
            name: row.get(1),
            is_group: row.get(2),
            users: row.get(3),
        }).collect();
    HttpResponse::Ok().json( ResponseJsonWithData {
        data: rooms,
        message:"success".to_string(),
        status:"success".to_string(),
        status_code:200
    })
}
