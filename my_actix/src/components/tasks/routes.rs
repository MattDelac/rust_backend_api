use actix_web::{error, body::BoxBody, get, delete, post, web::{self, Path}, http::header::ContentType, HttpResponse, Responder};
use diesel::{prelude::*, r2d2};

type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

use crate::components::tasks::model::{tasks, InputTask, Task, TaskFilter};


#[post("/")]
pub async fn create(pool: web::Data<DbPool>, task: web::Json<InputTask>) -> actix_web::Result<impl Responder> {
    let mut value = task.clone();
    value.created_at = chrono::Utc::now().naive_utc();
    value.updated_at = value.created_at;

    web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get().unwrap();

        diesel::insert_into(tasks::table)
            .values(value)
            .execute(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/read_filter")]
pub async fn list(pool: web::Data<DbPool>, filter: web::Json<TaskFilter>) -> actix_web::Result<impl Responder> {
    let mut query = tasks::table.into_boxed::<diesel::sqlite::Sqlite>();


    if let Some(description) = filter.description.clone() {
        if let Some(any_) = description.any_ {
            query = query.filter(tasks::description.eq_any(any_));
        }
        if let Some(like_) = description.like_.clone() {
            query = query.filter(tasks::description.like(format!("%{like_}%")));
        }
    }

    if let Some(completed) = filter.completed.clone() {
        query = query.filter(tasks::completed.eq(completed.eq_));
    }

    if let Some(created_at) = filter.created_at.clone() {
        if let Some(before_) = created_at.before_ {
            query = query.filter(tasks::created_at.le(before_));
        }
        if let Some(after_) = created_at.after_ {
            query = query.filter(tasks::created_at.ge(after_));
        }
        match created_at.is_null {
            true => query = query.filter(tasks::created_at.is_null()),
            false => query = query.filter(tasks::created_at.is_not_null()),
        }
    }

    if let Some(updated_at) = filter.updated_at.clone() {
        if let Some(before_) = updated_at.before_ {
            query = query.filter(tasks::updated_at.le(before_));
        }
        if let Some(after_) = updated_at.after_ {
            query = query.filter(tasks::updated_at.ge(after_));
        }
        match updated_at.is_null {
            true => query = query.filter(tasks::updated_at.is_null()),
            false => query = query.filter(tasks::updated_at.is_not_null()),
        }
    }
    
    let tasks = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get().unwrap();

        query.limit(20).load::<Task>(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .json(tasks))
}

#[get("/<id>")]
pub async fn get_single(pool: web::Data<DbPool>, id: Path<i32>) -> actix_web::Result<impl Responder> {
    let task = web::block(move || {
        // note that obtaining a connection from the pool is also potentially blocking
        let mut conn = pool.get().unwrap();

        tasks::table
            .filter(tasks::id.eq(id.into_inner()))
            .load::<Task>(&mut conn)
            // .first(mut &conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .json(task))
}

#[delete("/<id>")]
pub async fn delete(pool: web::Data<DbPool>, id: Path<i32>) -> actix_web::Result<impl Responder>  {
    web::block(move || {
        let mut conn = pool.get().unwrap();

        diesel::delete(tasks::table)
            .filter(tasks::id.eq(id.into_inner()))
            .execute(&mut conn)
    }).await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::NoContent().finish())
}
