use actix_web::{error, get, delete, web, HttpResponse, Responder};

use diesel::{prelude::*, r2d2};
use crate::components::tasks::model::tasks;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

#[get("")]
pub async fn hello_world() -> String {
    "Hello world!".to_string()
}

#[delete("")]
pub async fn destroy(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get().unwrap();

        diesel::delete(tasks::table).execute(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::NoContent().finish())
}