use rocket_sync_db_pools::{diesel, database};

#[database("sqlite")]
pub struct Db(diesel::SqliteConnection);