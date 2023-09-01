use actix_web::web::block;
use actix_web::{web, HttpResponse};
use chrono::NaiveDateTime;
use diesel::prelude::Queryable;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::{PgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::interface::global::ResponseJsonWithData;
use crate::{
    config::auth::Auth,
    interface::global::ResponseJson,
    models::{Users, Role},
    schema::users,
};

type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Deserialize, Serialize)]
pub struct NewUsers {
    pub name: String,
    pub nrp: String,
    pub password: String,
}

fn validator(args: &NewUsers) -> Result<(), String> {
    if args.name.len() == 0 {
        return Err(String::from("Name can not be empty"));
    }

    if args.name.len() > 40 {
        return Err(String::from("Nama tidak boleh lebih dari 40 char"));
    }

    if args.nrp.len() > 20 {
        return Err(String::from("Nama tidak boleh lebih dari 20 char"));
    }

    if args.nrp.len() == 0 {
        return Err(String::from("NRP Can not be empty"));
    }

    if args.password.len() == 0 {
        return Err(String::from("Password can not be empty"));
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Queryable, Clone)]
#[diesel(table_name = users)]
struct UsersResult {
    id: uuid::Uuid,
    nrp: String,
    name: String,
    created_at: Option<NaiveDateTime>,
}

#[allow(dead_code)]
pub async fn index(
    req: web::Json<NewUsers>,
    db: web::Data<DbPool>,
) -> HttpResponse {
    match validator(&req) {
        Ok(_) => {}
        Err(err) => {
            return HttpResponse::BadRequest().json(ResponseJson {
                message: err,
                status: "fail".to_string(),
                status_code: 400,
            })
        }
    }
    let auth = Auth {
        plain: req.password.to_string(),
    };
    let insert_into_db = Users {
        id: uuid::Uuid::new_v4(),
        name: req.name.to_string(),
        nrp: req.nrp.to_string(),
        password: auth.encrypt().into(),
        role: Role::MEMBER,
    };

    let _block = block(move || {
        let mut connection = db.get().expect("Something went wrong with database");

        let result = diesel::insert_into(users::table)
            .values(&insert_into_db)
            .returning((users::id, users::nrp, users::name, users::created_at))
            .get_result::<(uuid::Uuid, String, String, Option<NaiveDateTime>)>(&mut connection);

        match result {
            Ok(res) => Ok(res),
            Err(err) => {
                println!("{} ", err);
                Err("NRP telah digunakan")
            }
        }
    })
    .await
    .unwrap();

    let _block = match _block {
        Ok(res) => res,
        Err(err) => {
            println!("Error {}", err);
            return HttpResponse::BadRequest().json(ResponseJson {
                message: err.to_string(),
                status: "fail".to_string(),
                status_code: 400,
            });
        }
    };

    let (id, name, nrp, created_at) = _block;
    let response = UsersResult {
        id,
        name,
        created_at,
        nrp,
    };

    HttpResponse::Ok().json(ResponseJsonWithData {
        data: response,
        message: "success".to_string(),
        status: "success".to_string(),
        status_code: 200,
    })
}
