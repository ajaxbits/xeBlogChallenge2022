use actix_web::{error, web};
use chrono::NaiveDate;
use rusqlite::{named_params, Row};
use serde::{Deserialize, Serialize};

const DB_PATH: &str = "./posts.db";

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct Post {
    pub title: String,
    pub date: chrono::NaiveDate,
    pub slug: String,
    pub content: String,
}

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

pub async fn execute_get_post(
    pool: &Pool,
    date: String,
    slug: String,
) -> Result<Post, actix_web::Error> {
    let pool = pool.clone();
    let conn = web::block(move || pool.get())
        .await?
        .map_err(error::ErrorInternalServerError)?;

    web::block(move || get_post(conn, date, slug))
        .await?
        .map_err(error::ErrorInternalServerError)
}

pub async fn execute_get_post_index(pool: &Pool) -> Result<Vec<Post>, actix_web::Error> {
    let pool = pool.clone();
    let conn = web::block(move || pool.get())
        .await?
        .map_err(error::ErrorInternalServerError)?;

    web::block(move || get_posts(conn))
        .await?
        .map_err(error::ErrorInternalServerError)
}

fn get_posts(conn: Connection) -> Result<Vec<Post>, rusqlite::Error> {
    let mut statement = conn.prepare("SELECT * FROM posts")?;
    let rows = statement.query_map([], |row| {
        let date: String = row.get(1)?;
        Ok(Post {
            title: row.get(0)?,
            date: NaiveDate::parse_from_str(&date, "%Y-%m-%d")
                .expect("failed to parse the sql date to a NaiveDate"),
            slug: row.get(2)?,
            content: row.get(3)?,
        })
    })?;

    let mut posts = Vec::new();
    for post_result in rows {
        posts.push(post_result?);
    }
    Ok(posts)
}

fn get_post(conn: Connection, date: String, slug: String) -> Result<Post, rusqlite::Error> {
    let mut statement = conn.prepare("SELECT * FROM posts WHERE date = :date AND slug = :slug")?;
    let post: Post =
        statement.query_row(named_params! { ":date": date, ":slug": slug }, |row| {
            Ok(Post {
                title: row.get(0)?,
                date: NaiveDate::parse_from_str(&date, "%Y-%m-%d")
                    .expect("failed to parse the sql date to a NaiveDate"),
                slug: row.get(2)?,
                content: row.get(3)?,
            })
        })?;
    Ok(post)
}

