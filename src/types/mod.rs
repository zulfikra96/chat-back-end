
use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::Pool;

#[allow(dead_code)]
pub type DbPool = Pool<ConnectionManager<PgConnection>>;
