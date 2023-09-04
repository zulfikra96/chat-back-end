use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use deadpool_postgres::{ManagerConfig, RecyclingMethod, Pool as TokioPool, Manager };
use tokio_postgres::NoTls;
// use super::interface::global::;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

#[allow(dead_code)]
pub fn establish_connection() -> PgConnection{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Database url must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}",database_url))
}

/**
 * Asyn connection pool
 */
#[allow(dead_code)]
pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Database url must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not connect")
        
}

/**
 * Database connection configuration
 */
#[allow(dead_code)]
pub fn async_connection() -> TokioPool {
    dotenv().ok();
    let mut pg_config = tokio_postgres::Config::new();
    let db_name = env::var("POSTGRES_DB").expect("POSTGRES DB must be set");
    let db_username = env::var("POSTGRES_USER").expect("POSTGRES USER must be set");
    let db_host = env::var("POSTGRES_HOST").expect("POSTGRES HOST must be set");
    let db_password = env::var("POSTGRES_PASSWORD").expect("Postgres password must be set");
    let db_port = env::var("POSTGRES_PORT").expect("PORT  must be set");
    pg_config.dbname(&db_name);
    pg_config.host(&db_host);
    pg_config.user(&db_username);
    pg_config.password(&db_password);
    pg_config.port(db_port.parse::<u16>().unwrap());
    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast
    };

    let mgr = Manager::from_config(pg_config, NoTls, mgr_config);
    TokioPool::builder(mgr).max_size(16).build().unwrap()

}