use std::future::{ready, Ready};
use actix_web::dev::{ServiceRequest, ServiceResponse, Service};
use actix_web::{Error, HttpResponse, error};
use actix_web::dev::{forward_ready,Transform};
use super::super::controllers::auth::login::ErrorResponse;
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::interface::global::UserToken;

use actix_web::http::header::{self, HeaderValue };
use futures_util::FutureExt;
use futures_util::future::LocalBoxFuture;

#[derive(Debug)]
pub enum Status {
    SUCCESS,
    FAIL
}

impl error::ResponseError for ErrorResponse {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::Unauthorized().json(ErrorResponse {
            message: self.message.to_string(),
            status_code: self.status_code,
            status:self.status.to_string()
        })
    }
}


// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
#[derive(Debug)]

pub struct Authorize {
    pub role: String
}

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Authorize
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SayHiMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SayHiMiddleware { service, role: self.role.to_string() }))
    }
}

#[derive(Debug)]
pub struct SayHiMiddleware<S> {
    service: S,
    role: String
}


impl<S, B> Service<ServiceRequest> for SayHiMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
    
    
    
    forward_ready!(service);
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let headers = {
            req.headers()
        };
        let authorization = headers.get(header::AUTHORIZATION);
        let binding = HeaderValue::from_static("") ;

        let str = match authorization {
            Some(_str) => _str,
            None => &binding                     
        };
        let response  = move |req , status: Status|  {
           
            self.service.call(req).map(move |res| {
                
                match status {
                    Status::SUCCESS => res,
                    Status::FAIL => Ok(Err(ErrorResponse { 
                        message:"Unauthorized".to_string(),
                        status_code: 401,
                        status:"fail".to_string()
                     })?)
                }

            })
        };
        
        // println!("call HERE {}", str.len());
        if str.len() == 0 {
        println!("Exec error");

            return Box::pin(response(req, Status::FAIL));
        }

        let auth_header = str.to_str().unwrap();
        let spliter: Vec<&str> = auth_header.split(" ").collect();
        dotenv::dotenv().ok();
        let token = spliter[1];
        let key = std::env::var("PRIVATE_KEY").expect("Private key not found").to_string();
        let verify_token = decode::<UserToken>(token, &DecodingKey::from_secret(key.as_ref()), &Validation::new(jsonwebtoken::Algorithm::HS256));
        
        
        let plain = match verify_token {
            Ok(token_data) => token_data,
            Err(_) => {
                return Box::pin(response(req, Status::FAIL))
            }
        };  
        println!("still call header");
        
        if plain.claims.role != self.role {
            return Box::pin(response(req, Status::FAIL));
        }

        Box::pin(response(req, Status::SUCCESS))

    }
}




#[allow(dead_code)]
pub fn auth_fn(req: ServiceRequest, role: &str) -> Result<ServiceRequest, (ServiceRequest, ErrorResponse)> {
    let headers = { req.headers() };
    let authorization = headers.get(header::AUTHORIZATION);
    let binding = HeaderValue::from_static("");

    let str = match authorization {
        Some(_str) => _str,
        None => &binding,
    };

    // println!("call HERE {}", str.len());
    if str.len() == 0 {
        return Err((req, ErrorResponse {
            message: "Unauthorized".to_string(),
            status: "fail".to_string(),
            status_code: 401,
        }));
    }

    let auth_header = str.to_str().unwrap();
    let spliter: Vec<&str> = auth_header.split(" ").collect();
    dotenv::dotenv().ok();
    if spliter.len() < 2 {
        return Err((req, ErrorResponse {
            message: "Unauthorized".to_string(),
            status: "fail".to_string(),
            status_code: 401,
        }));
    }
    let token = spliter[1];
    let key = std::env::var("PRIVATE_KEY")
        .expect("Private key not found")
        .to_string();
    let verify_token = decode::<UserToken>(
        token,
        &DecodingKey::from_secret(key.as_ref()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    );

    let plain = match verify_token {
        Ok(token_data) => token_data,
        Err(_) => {
            return Err((req, ErrorResponse {
                message: "Unauthorized".to_string(),
                status: "fail".to_string(),
                status_code: 401,
            }))
        }
    };

    if plain.claims.role != role {
        return Err((req, ErrorResponse {
            message: "Unauthorized".to_string(),
            status: "fail".to_string(),
            status_code: 401,
        }));
    }

    Ok(req)
}
