use serde::{Serialize, Deserialize};
use serde_valid::Validate;

#[derive(Debug, Clone, Validate, Deserialize, Serialize)]
pub struct TextFilter {
    pub any_: Option<Vec<String>>,
    
    #[validate(min_length = 1)]
    #[validate(max_length = 255)]
    pub like_: Option<String>,
}

#[derive(Debug, Clone, Validate, Deserialize, Serialize)]
pub struct BoolFilter {
    pub eq_: bool,
}

#[derive(Debug, Clone, Validate, Deserialize, Serialize)]
pub struct TimestampFilter {
    pub before_: Option<chrono::NaiveDateTime>,
    pub after_: Option<chrono::NaiveDateTime>,
    #[serde(default)]
    pub is_null: bool,
}