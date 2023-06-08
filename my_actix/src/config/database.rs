use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::result::Error;
use std::thread;

pub fn get_connection_pool() -> Pool<ConnectionManager<DbConnection>> {
    let url = database_url_for_env();
    let manager = ConnectionManager::<DbConnection>::new(url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}