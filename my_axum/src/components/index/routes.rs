use axum::{extract::State, Json, http::StatusCode};
use diesel::RunQueryDsl;

use crate::config::database::Pool;
use crate::components::tasks::model::tasks;
use crate::utils::errors::internal_error;


pub async fn hello_world() -> Result<&'static str, (StatusCode, String)> {
    Ok("Hello, world!")
}


pub async fn destroy(State(pool): State<Pool>) -> Result<usize, (StatusCode, String)> {
    let mut conn = pool.get().map_err(internal_error)?;
    let res = diesel::delete(tasks::table).execute(&mut conn).map_err(internal_error)?;

    Ok(res)
}