use std::fmt;

use super::super::super::interface::global::ResponseJson;
use crate::interface::global::{UserToken, ResponseJsonWithData};
use crate::models::Users;
use crate::models::Role;
use actix_web::{error, web, HttpResponse};
use derive_more::{Display, Error};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::ExpressionMethods;
use diesel::PgConnection;
use dotenv::dotenv;
use jsonwebtoken::{encode, EncodingKey, Header};
use magic_crypt::new_magic_crypt;
use magic_crypt::MagicCryptTrait;
use serde::{Deserialize, Serialize};
use chrono::Utc;
use uuid::Uuid;

#[allow(dead_code)]
type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Deserialize, Clone)]
pub struct LoginParams {
    pub nrp: String,
    pub password: String,
}

impl Role {
    #[allow(dead_code)]
    fn from_str(&self) -> Result<String,()> {
        match self {
            Self::ADMIN => Ok("ADMIN".to_string()),
            Self::MEMBER => Ok("MEMBER".to_string()),
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::ADMIN => write!(f, "ADMIN"),
            Role::MEMBER => write!(f, "MEMBER")
        }
    }
}

#[derive(Debug, Display, Error, Serialize, Deserialize)]
#[display(fmt = "Error not found")]
pub struct ErrorResponse {
    pub status_code: u32,
    pub message: String,
    pub status: String,
}

#[derive(Debug, Serialize)]
struct DataResponse {
    token: String,
    id: Uuid,
    nrp: String,
    role: Role
}

#[derive(Debug, Serialize)]
struct SuccessResponse {
    status: String,
    message: String,
    data: DataResponse,
}

#[allow(dead_code)]
pub fn validator(login_params: &web::Json<LoginParams>) -> Result<(), ResponseJson> {
    if login_params.nrp.len() > 20 {
        return Err(ResponseJson {
            status: String::from("fail"),
            status_code: 400,
            message: String::from("Nrp tidak boleh lebih dari 20 karakter"),
        });
    }

    Ok(())
}
#[allow(dead_code)]
fn select_users(
    conn: &mut PgConnection,
    _nrp: String,
) -> Result<std::option::Option<Users>, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    // use crate::models::Users;

    let results = users
        .filter(nrp.eq(&_nrp))
        .select(Users::as_select())
        .first(conn)
        .optional();
    return results;
}

impl error::ResponseError for ResponseJson {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::NotFound().json(ResponseJson {
            message: self.message.to_string(),
            status_code: self.status_code,
            status: self.status.to_string(),
        })
    }
}

#[allow(dead_code)]
pub async fn login_controller(
    request_body: web::Json<LoginParams>,
    db: web::Data<DbPool>,
) -> Result<HttpResponse, ResponseJson> {
    // let _request_body = &request_body;
    let _validator = validator(&request_body);
    let _request = request_body.clone();
    match _validator {
        Ok(()) => (),
        Err(e) => return Err(e),
    }

    let get_user = web::block(move || {
        let mut connection = db.get().expect("Could not connect db");
        select_users(&mut connection, request_body.into_inner().nrp)
    })
    .await
    .unwrap()
    .map_err(error::ErrorInternalServerError)
    .unwrap();

    dotenv().ok();

    let user = match get_user {
        Some(result) => result,
        None => {
            return Err(ResponseJson {
                message: "Invalid password or NRp".to_string(),
                status: "fail".to_string(),
                status_code: 422,
            })
        }
    };

    let private_key = std::env::var("PRIVATE_KEY").expect("Private key is not set");
    let mc = new_magic_crypt!(&private_key, 256);
    let expiration = Utc::now().checked_add_signed(chrono::Duration::days(1)).expect("valid timestamp").timestamp();
    let password = user.password.unwrap();
    let token_binding = UserToken {
        id: user.id,
        name: user.name,
        nrp: user.nrp,
        role: user.role.from_str().unwrap(),
        exp: expiration as usize
    };
    let token = encode(
        &Header::default(),
        &token_binding,
        &EncodingKey::from_secret(private_key.as_ref()),
    )
    .unwrap();
    let success_response = ResponseJsonWithData {
        data: DataResponse {
            token,
            id: user.id,
            nrp: token_binding.nrp,
            role: user.role
        },
        message: "Success to login".to_string(),
        status: "success".to_string(),
        status_code:200
    };
    if mc.decrypt_base64_to_string(&password).unwrap() != _request.password {
        return Err(ResponseJson {
            message: String::from("Invalid password"),
            status: String::from("fail"),
            status_code: 401,
        });
    }
    let  _response = HttpResponse::Ok().json(success_response);
    Ok(_response)
        
}
