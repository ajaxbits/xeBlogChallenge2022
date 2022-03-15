use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct Post {
    pub title: String,
    pub date: chrono::NaiveDate,
    pub slug: String,
    pub content: String,
}

impl Post {
    /// a debug method used to create arbitrary posts
    pub fn new(title: &str, date: &str, slug: &str, content: &str) -> Self {
        Self {
            title: title.to_string(),
            date: chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d")
                .expect("failed to parse date string"),
            slug: slug.to_string(),
            content: content.to_string(),
        }
    }

    pub async fn all(pool: &SqlitePool) -> Result<Vec<Post>, sqlx::Error> {
        let posts = sqlx::query_as!(
            Post,
            r#"SELECT title, date as "date!: chrono::NaiveDate", slug, content FROM posts"#
        )
        .fetch_all(pool)
        .await?;
        Ok(posts)
    }

    pub async fn insert(new_post: Post, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        sqlx::query_as!(
            Post,
            r#"
            INSERT INTO posts
            VALUES ($1,$2,$3,$4)
            "#,
            new_post.title,
            new_post.date,
            new_post.slug,
            new_post.content
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get(
        date: chrono::NaiveDate,
        slug: String,
        pool: &SqlitePool,
    ) -> Result<Post, sqlx::Error> {
        let post = sqlx::query_as!(
            Post,
            r#"
            SELECT title, date as "date!: chrono::NaiveDate", slug, content 
            FROM posts
            WHERE date=$1 AND slug=$2
            "#,
            date,
            slug,
        )
        .fetch_one(pool)
        .await?;
        Ok(post)
    }

    pub async fn update_with_slug(
        og_post_slug: String,
        new_post: Post,
        pool: &SqlitePool,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE posts
            SET title=$1, date=$2, slug=$3, content=$4
            WHERE (slug=$6)
            "#,
            new_post.title,
            new_post.date,
            new_post.slug,
            new_post.content,
            og_post_slug,
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
