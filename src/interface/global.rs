use serde::{ Deserialize, Serialize };
use derive_more::{ Display, Error };
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Display, Error )]
#[display(fmt = "{}", message)]
pub struct ResponseJson {
    pub status: String,
    pub status_code: u32,
    pub message: String
}

#[derive(Deserialize, Serialize, Debug, Display, Error )]
#[display(fmt = "{}", message)]
pub struct ResponseJsonWithData<T> {
    pub status: String,
    pub status_code: u32,
    pub message: String,
    pub data: T
}


#[derive(Deserialize, Debug, Serialize)]
pub struct UserToken {
    pub nrp: String,
    pub name: String,
    pub id: Uuid,
    pub role: String,
    pub exp: usize
}
