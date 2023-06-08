use rocket::{delete, get};
use rocket_sync_db_pools::diesel;
use diesel::RunQueryDsl;

use crate::config::database::Db;
use crate::utils::errors::Result;
use crate::components::tasks::model::tasks;

#[get("/")]
pub async fn hello_world() -> Result<&'static str> {
    Ok("Hello, world!")
}

#[delete("/")]
pub async fn destroy(db: Db) -> Result<()> {
    db.run(move |conn| diesel::delete(tasks::table).execute(conn)).await?;

    Ok(())
}