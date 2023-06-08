use rocket::response::Debug;
use rocket_sync_db_pools::diesel;

pub type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;