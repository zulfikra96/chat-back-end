// use crate::schema::{ users, Role, RoleType };
use diesel::expression::AsExpression;
use diesel::pg::Pg;
use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::prelude::*;
use diesel::serialize::{ ToSql, IsNull };
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::io::Write;
use diesel::serialize::{self, Output};
use diesel::backend::Backend;

use crate::schema::sql_types::RoleType;
use crate::schema::users;


#[derive(Debug, AsExpression, Clone, Copy, FromSqlRow, PartialEq, Eq, Deserialize, Serialize)]
#[diesel(sql_type = RoleType)]
pub enum Role {
    ADMIN,
    MEMBER
}


impl  ToSql<RoleType, Pg> for Role{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
       match *self {
        Role::ADMIN => out.write_all(b"ADMIN").unwrap(),
        Role::MEMBER => out.write_all(b"MEMBER").unwrap()
       }
       Ok(IsNull::No)
    }
}

impl FromSql<RoleType, Pg> for Role {
    fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"ADMIN" =>Ok(Role::ADMIN),
            b"MEMBER" => Ok(Role::MEMBER),
            _=> Err("Uncategorize enum".into())
        }
    }
}


#[derive(Queryable, Selectable, Debug, Clone, Insertable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {
    pub id: Uuid,
    pub nrp: String,
    pub name: String,
    pub password: Option<String>,
    pub role: Role,
}
