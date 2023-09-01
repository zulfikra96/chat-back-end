mod config;
mod models;
mod controllers;
mod schema;
mod interface;
mod types;
mod socket_server;
mod session;
use serde::Serialize;
mod  messages;
use tokio_postgres::types::{ FromSql, ToSql };

#[derive(Debug, ToSql, FromSql, Clone, Serialize)]
#[allow(non_camel_case_types)]
enum role_type {
    ADMIN,
    MEMBER
}


#[cfg(test)]
mod tests {
    
    use std::env;
    
    use magic_crypt::{new_magic_crypt, MagicCryptTrait};
    use dotenv::dotenv;
    use crate::config::database::async_connection;
    use crate::role_type;
    
    #[tokio::test]
    async fn create_new_admin() {
        dotenv().ok();
        let private_key = env::var("PRIVATE_KEY").expect("Private key is not set");
        let mc = new_magic_crypt!(&private_key,256);
        let chipper = mc.encrypt_str_to_base64("Hello world");
        println!("{}", mc.decrypt_base64_to_string(&chipper).unwrap());
        let client = async_connection().get().await.unwrap();
        let nrp = "1234/p";
        let name = "ZUlfikra";
        let password = chipper;
        let role = role_type::ADMIN;
        let exec = client.execute("INSERT INTO users (nrp, name, password, role) VALUES($1, $2, $3, $4)", &[&nrp,&name, &password, &role]).await;

        match exec {
            Ok(_) => println!("Success to insert"),
            Err(e) => println!("{}", e)
        }
    }
}
