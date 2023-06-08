use rocket::serde::{Serialize, Deserialize};
use serde_valid::Validate;
use rocket_sync_db_pools::diesel;

use self::diesel::prelude::*;
use crate::utils::filters::{TextFilter, BoolFilter, TimestampFilter};

#[derive(Debug, Clone, Validate, Serialize, Deserialize, Insertable, Queryable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = tasks)]
pub struct Task {
    /// The task's unique ID.
    #[serde(skip_deserializing)]
    #[validate(minimum = 0)]
    pub id: i32,

    /// The task's description.
    /// Has a maximum length of 255 characters.
    #[validate(max_length = 255)]
    pub description: String,
    
    /// Whether the task is completed or not.
    #[serde(skip_deserializing)]
    pub completed: bool,
    
    /// The task's creation date.
    #[serde(skip_deserializing)]
    pub created_at: chrono::NaiveDateTime,
    
    /// The task's last update date.
    #[serde(skip_deserializing)]
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Validate, Deserialize, Serialize)]
pub struct TaskFilter {
    /// Page to use as an offset
    #[serde(default)]
    #[validate(minimum = 0)]
    pub page: i64,
    
    /// Number of items to fetch during a single iteration
    #[serde(default)]
    #[validate(minimum = 0)]
    #[validate(maximum = 250)]
    pub limit: i64,

    // #[serde(default = "and_")]
    pub operator: String,
    pub description: Option<TextFilter>,
    pub completed: Option<BoolFilter>,
    pub created_at: Option<TimestampFilter>,
    pub updated_at: Option<TimestampFilter>,
}

table! {
    tasks (id) {
        id -> Integer,
        description -> Text,
        completed -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
