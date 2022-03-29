use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct Post {
    #[serde(default)]
    pub uuid: Uuid,

    #[serde(default)]
    pub draft: bool,

    pub title: String,

    pub date: chrono::NaiveDate,

    #[serde(default = "today")]
    pub updated: chrono::NaiveDate,

    pub slug: String,

    #[serde(default = "Vec::new")]
    pub tags: Vec<String>,

    pub content: Option<String>,
}

#[derive(Serialize, Clone, Deserialize, Debug)]
struct RawPost {
    pub uuid: Vec<u8>,
    pub draft: i64,
    pub title: String,
    pub date: String,
    pub updated: String,
    pub slug: String,
    pub tags: String,
    pub content: Option<String>,
}

fn today() -> chrono::NaiveDate {
    chrono::offset::Local::today().naive_local()
}

impl From<RawPost> for Post {
    fn from(raw_post: RawPost) -> Self {
        Post {
            uuid: Uuid::from_slice(&raw_post.uuid).expect("could not parse uuid"),
            title: raw_post.title,
            draft: match &raw_post.draft {
                0 => false,
                1 => true,
                _ => panic!(),
            },
            date: chrono::NaiveDate::parse_from_str(&raw_post.date, "%Y-%m-%d")
                .expect("failed to parse date string"),
            updated: chrono::NaiveDate::parse_from_str(&raw_post.updated, "%Y-%m-%d")
                .expect("failed to parse date string"),
            slug: raw_post.slug,
            tags: raw_post.tags.split(',').map(|str| str.to_owned()).collect(),
            content: raw_post.content,
        }
    }
}

impl Post {
    pub async fn all(pool: &SqlitePool) -> Result<Vec<Post>, sqlx::Error> {
        let posts = sqlx::query_as!(
            RawPost,
            r#"
            SELECT 
                uuid,
                draft,
                title, 
                date,
                updated,
                slug, 
                tags, 
                content 
            FROM posts
            "#
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|raw_post| Post::from(raw_post))
        .collect();

        Ok(posts)
    }

    pub async fn insert(new_post: Post, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        let new_uuid = Uuid::new_v4();
        let tags: String = new_post.tags.into_iter().collect();
        let draft = false;
        sqlx::query!(
            r#"
            INSERT INTO posts
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8)
            "#,
            new_uuid,
            draft,
            new_post.title,
            new_post.date,
            new_post.updated,
            new_post.slug,
            tags,
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
            RawPost,
            r#"
            SELECT uuid, draft, title, date, updated, slug, tags, content 
            FROM posts
            WHERE date=$1 AND slug=$2
            "#,
            date,
            slug,
        )
        .fetch_one(pool)
        .await?
        .into();

        Ok(post)
    }

    pub async fn get_with_uuid(uuid: Uuid, pool: &SqlitePool) -> Result<Post, sqlx::Error> {
        let post: Post = sqlx::query_as!(
            RawPost,
            r#"
            SELECT uuid, draft, title, date, updated, slug, tags, content 
            FROM posts
            WHERE uuid=$1
            "#,
            uuid
        )
        .fetch_one(pool)
        .await?
        .into();

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
