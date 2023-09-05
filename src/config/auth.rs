use std::env;

use actix_web::http::{self, header::HeaderMap};
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation, errors::{Error, ErrorKind}};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

use crate::interface::global::UserToken;

pub struct Auth {
    pub plain: String,
}

pub struct JWT<'a> {
    pub headers: &'a HeaderMap,
    pub token: &'a Option<String>
}

impl Auth {
    pub fn encrypt(&self) -> String {
        dotenv::dotenv().ok();
        let private_key = env::var("PRIVATE_KEY").expect("Private key is not set");
        let mc = new_magic_crypt!(&private_key, 256);
        mc.encrypt_str_to_base64(&self.plain)
    }
}

impl<'a> JWT<'a> {
    pub fn decode_token(&self) -> Result<TokenData<UserToken>, jsonwebtoken::errors::Error> {
        let auth = self.headers.get(http::header::AUTHORIZATION);
        let private_token = env::var("PRIVATE_KEY").expect("private is not defied");

        let header = match auth {
            Some(_header) => Ok(_header),
            None => Err(println!("token is not set")),
        };
        let extract_token: Vec<String> = header
            .unwrap()
            .to_str()
            .unwrap()
            .split(" ")
            .into_iter()
            .map(|res| res.to_string())
            .collect();
        let token = extract_token.get(1);
        let decode_token = decode::<UserToken>(
            token.unwrap(),
            &DecodingKey::from_secret(private_token.as_ref()),
            &Validation::new(jsonwebtoken::Algorithm::HS256),
        );
        decode_token
    }

    pub fn decode_token_directly(&self) -> Result<TokenData<UserToken>, jsonwebtoken::errors::Error> {
        let private_key = env::var("PRIVATE_KEY").expect("private is not defied");
        if let Some(res) = self.token {
            let decode_token = decode(&res, &DecodingKey::from_secret(private_key.as_ref()), &Validation::new(jsonwebtoken::Algorithm::HS256));
            decode_token
        } else {
            Err(Error::from(ErrorKind::InvalidToken))
        }
    }
}


