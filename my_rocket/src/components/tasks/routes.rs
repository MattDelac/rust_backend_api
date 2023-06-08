use rocket::{post, get, delete};
use rocket::serde::json::Json;

use rocket_sync_db_pools::diesel;

use self::diesel::prelude::*;

use crate::config::database::Db;
use crate::utils::errors::Result;
use crate::components::tasks::model::{tasks, Task, TaskFilter};


#[post("/", data = "<task>")]
pub async fn create(db: Db, task: Json<Task>) -> Result<()> {
    let mut value = task.clone();
    value.created_at = chrono::Utc::now().naive_utc();
    value.updated_at = value.created_at;

    db.run(move |conn| {
        diesel::insert_into(tasks::table)
            .values(&*value)
            .execute(conn)
    }).await?;

    Ok(())
}

#[post("/read_filter", data = "<filter>")]
pub async fn list(db: Db, filter: Json<TaskFilter>) -> Result<Json<Vec<Task>>> {
    let mut query = tasks::table.into_boxed::<diesel::sqlite::Sqlite>();

    query = query.limit(filter.limit).offset(filter.page);


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
    
    let tasks: Vec<Task> = db.run(move |conn| {
        query.load::<Task>(conn)
    }).await?;

    Ok(Json(tasks))
}

#[get("/<id>")]
pub async fn get_single(db: Db, id: i32) -> Option<Json<Task>> {
    db.run(move |conn| {
        tasks::table
            .filter(tasks::id.eq(id))
            .first(conn)
    }).await.map(Json).ok()
}

#[delete("/<id>")]
pub async fn delete(db: Db, id: i32) -> Result<Option<()>> {
    let affected = db.run(move |conn| {
        diesel::delete(tasks::table)
            .filter(tasks::id.eq(id))
            .execute(conn)
    }).await?;

    Ok((affected == 1).then(|| ()))
}
