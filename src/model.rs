use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct Post {
    #[serde(default)]
    pub uuid: Uuid,
    pub title: String,
    pub date: chrono::NaiveDate,
    #[serde(default = "default_to_today")]
    pub updated: chrono::NaiveDate,
    pub slug: String,
    // TODO implement and add to db, store as BLOB
    // #[serde(default = "Vec::new")]
    // pub tags: Vec<String>,
    pub content: Option<String>,
}

fn default_to_today() -> chrono::NaiveDate {
    chrono::offset::Local::today().naive_local()
}

impl Post {
    /// a debug method used to create arbitrary posts
    pub fn _new(title: &str, date: &str, slug: &str, content: &str) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            title: title.to_string(),
            date: chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d")
                .expect("failed to parse date string"),
            updated: chrono::offset::Local::today().naive_local(),
            slug: slug.to_string(),
            content: Some(content.to_string()),
        }
    }

    pub async fn all(pool: &SqlitePool) -> Result<Vec<Post>, sqlx::Error> {
        let posts = sqlx::query_as!(
            Post,
            r#"SELECT uuid as "uuid!: uuid::Uuid", title, date as "date!: chrono::NaiveDate", updated as "updated!: chrono::NaiveDate", slug, content FROM posts"#
        )
        .fetch_all(pool)
        .await?;
        Ok(posts)
    }

    pub async fn insert(new_post: Post, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        let new_uuid = Uuid::new_v4();
        sqlx::query_as!(
            Post,
            r#"
            INSERT INTO posts
            VALUES ($1,$2,$3,$4,$5, $6)
            "#,
            new_uuid,
            new_post.title,
            new_post.date,
            new_post.updated,
            new_post.slug,
            new_post.content
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn delete(post: Post, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM posts
            WHERE uuid=$1
            "#,
            post.uuid
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_with_dateslug(
        date: chrono::NaiveDate,
        slug: String,
        pool: &SqlitePool,
    ) -> Result<Post, sqlx::Error> {
        let post: Post = sqlx::query_as!(
            Post,
            r#"
            SELECT uuid as "uuid!: uuid::Uuid", title, date as "date!: chrono::NaiveDate", updated as "updated!: chrono::NaiveDate", slug, content
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

    pub async fn get_with_uuid(uuid: Uuid, pool: &SqlitePool) -> Result<Post, sqlx::Error> {
        let post: Post = sqlx::query_as!(
            Post,
            r#"
            SELECT uuid as "uuid!: uuid::Uuid", title, date as "date!: chrono::NaiveDate", updated as "updated!: chrono::NaiveDate", slug, content
            FROM posts
            WHERE uuid=$1
            "#,
            uuid
        )
        .fetch_one(pool)
        .await?;

        Ok(post)
    }

    pub async fn update(
        old_post: Post,
        new_post: Post,
        pool: &SqlitePool,
    ) -> Result<(), sqlx::Error> {
        Post::delete(old_post, pool).await?;
        Post::insert(new_post, pool).await?;
        Ok(())
    }
}
