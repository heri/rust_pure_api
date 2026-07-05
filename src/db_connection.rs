use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, PooledConnection};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

fn database_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set - View README")
}

pub fn establish_connection_pool() -> Arc<DbPool> {
    let manager = ConnectionManager::<PgConnection>::new(database_url());
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    Arc::new(pool)
}

pub fn get_connection(pool: &DbPool) -> Result<DbConnection, diesel::result::Error> {
    pool.get()
        .map_err(|e| diesel::result::Error::QueryBuilderError(Box::new(e)))
}