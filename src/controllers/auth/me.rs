
use crate::config::auth::JWT;
use crate::interface::global::ResponseJsonWithData;
use actix_web::{HttpRequest, HttpResponse};

use super::login::ErrorResponse;
// use super::controllers::auth::login::{ ErrorResponse, UserToken };

#[allow(dead_code)]
pub async fn index(req: HttpRequest) -> HttpResponse {
    let auth = JWT {
        headers: req.headers(),
    };
    let decode_token = auth.decode_token();

    let token_data = match decode_token {
        Ok(data) => data,
        Err(err) => {
            return {
                println!("Error : {}", err);
                HttpResponse::BadRequest().json(ErrorResponse {
                    message: "Something went wrong".to_string(),
                    status_code: 400,
                    status: "fail".to_string(),
                })
            }
        }
    };
    let user_data = token_data.claims;

    let response = ResponseJsonWithData {
        data: user_data,
        message: "".to_string(),
        status: "success".to_string(),
        status_code: 200,
    };
    HttpResponse::Ok().json(response)
}
