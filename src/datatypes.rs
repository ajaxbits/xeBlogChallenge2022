use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    title: String,
    date: chrono::NaiveDate,
    slug: String,
    content: String,
}
