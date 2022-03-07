use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub title: String,
    pub date: chrono::NaiveDate,
    pub slug: String,
    pub content: String,
}
