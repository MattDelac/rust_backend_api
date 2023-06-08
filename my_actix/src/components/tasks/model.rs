use serde::{Serialize, Deserialize};
use serde_valid::Validate;

use diesel::prelude::*;
use crate::utils::filters::{TextFilter, BoolFilter, TimestampFilter};

#[derive(Debug, Clone, Validate, Deserialize, Insertable)]
#[diesel(table_name = tasks)]
pub struct InputTask {
    /// The task's description.
    /// Has a maximum length of 255 characters.
    #[validate(max_length = 255)]
    pub description: String,

    /// The task's creation date.
    #[serde(skip_deserializing)]
    pub created_at: chrono::NaiveDateTime,
    
    /// The task's last update date.
    #[serde(skip_deserializing)]
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Validate, Serialize, Queryable)]
#[diesel(table_name = tasks)]
pub struct Task {
    /// The task's unique ID.
    #[validate(minimum = 0)]
    pub id: i32,

    /// The task's description.
    /// Has a maximum length of 255 characters.
    #[validate(max_length = 255)]
    pub description: String,
    
    /// Whether the task is completed or not.
    pub completed: bool,
    
    /// The task's creation date.
    pub created_at: chrono::NaiveDateTime,
    
    /// The task's last update date.
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Validate, Deserialize, Serialize)]
pub struct TaskFilter {
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