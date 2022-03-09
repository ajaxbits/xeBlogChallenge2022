use chrono::NaiveDate;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct Post {
    pub title: String,
    pub date: chrono::NaiveDate,
    pub slug: String,
    pub content: String,
}

const DB_PATH: &str = "./posts.db";

pub fn get_posts() -> rusqlite::Result<Vec<Post>> {
    let conn = Connection::open(&DB_PATH)?;
    let mut stmt = conn.prepare("SELECT title, date, slug, content FROM posts")?;
    let rows = stmt.query_map([], |row| {
        let date: String = row.get(1).expect("failed to get date from sqlite");
        Ok(Post {
            title: row.get(0)?,
            date: NaiveDate::parse_from_str(&date, "%Y-%m-%d")
                .expect("failed to parse the sql date to a NaiveDate"),
            slug: row.get(2)?,
            content: row.get(3)?,
        })
    })?;
    let mut posts = Vec::new();
    for post in rows {
        posts.push(post?);
    }

    Ok(posts)
}
