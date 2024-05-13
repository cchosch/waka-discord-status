use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::{Object, Pool};
use rand::random;

use crate::VARS;

pub type ConnPool = Pool<AsyncPgConnection>;
pub type DbConn = Object<AsyncPgConnection>;

/// Generates random 8 byte integer encod as hex
pub fn gen_id() -> String {
    hex::encode(random::<[u8; 8]>())
}

/// Creates new `Pool<ConnectionManager<PgConnection>>>`
pub(crate) fn gen_pool() -> ConnPool {
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(VARS.clone().database_url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    let pool = Pool::builder(manager).build().unwrap();
    pool
}
