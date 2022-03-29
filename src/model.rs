use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct Post {
    #[serde(default)]
    pub uuid: Uuid,

    #[serde(default)]
    #[serde(deserialize_with = "bool_from_int")]
    pub draft: bool,

    pub title: String,

    pub date: chrono::NaiveDate,

    #[serde(default = "default_to_today")]
    pub updated: chrono::NaiveDate,

    pub slug: String,

    #[serde(default = "Vec::new")]
    pub tags: Vec<String>,

    pub content: Option<String>,
}

#[derive(Serialize, Clone, Deserialize, Debug)]
struct RawPost {
    #[serde(default)]
    pub uuid: Uuid,
    #[serde(default)]
    #[serde(deserialize_with = "bool_from_int")]
    pub draft: bool,
    pub title: String,
    pub date: chrono::NaiveDate,
    #[serde(default = "default_to_today")]
    pub updated: chrono::NaiveDate,
    pub slug: String,
    pub tags: String,
    pub content: Option<String>,
}

fn default_to_today() -> chrono::NaiveDate {
    chrono::offset::Local::today().naive_local()
}

/// Used to convert 0 and 1 to false and true, respectively
fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    // let test = 0 as u8;
    // TODO change back to u8 once you insert it into
    match i64::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(serde::de::Error::invalid_value(
            serde::de::Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}

impl From<RawPost> for Post {
    fn from(raw_post: RawPost) -> Self {
        Post {
            uuid: raw_post.uuid,
            draft: raw_post.draft,
            title: raw_post.title,
            date: raw_post.date,
            updated: raw_post.updated,
            slug: raw_post.slug,
            tags: raw_post
                .tags
                .split(',')
                .map(|str| str.to_owned())
                .collect::<Vec<String>>(),
            content: raw_post.content,
        }
    }
}

impl Post {
    /// a debug method used to create arbitrary posts
    pub fn _new(title: &str, date: &str, slug: &str, content: &str) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            title: title.to_string(),
            draft: false,
            date: chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d")
                .expect("failed to parse date string"),
            updated: default_to_today(),
            slug: slug.to_string(),
            tags: Vec::new(),
            content: Some(content.to_string()),
        }
    }

    pub async fn all(pool: &SqlitePool) -> Result<Vec<Post>, sqlx::Error> {
        let posts = sqlx::query_as!(
            RawPost,
            r#"
            SELECT 
                uuid as "uuid!: uuid::Uuid", 
                draft as "draft!: bool", 
                title, 
                date as "date!: chrono::NaiveDate", 
                updated as "updated!: chrono::NaiveDate", 
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
        let post: Post = Post::from(
            sqlx::query_as!(
                RawPost,
                r#"
            SELECT 
                uuid as "uuid!: uuid::Uuid", 
                draft as "draft!: bool", 
                title, 
                date as "date!: chrono::NaiveDate", 
                updated as "updated!: chrono::NaiveDate", 
                slug, 
                tags,
                content 
            FROM posts
            WHERE date=$1 AND slug=$2
            "#,
                date,
                slug,
            )
            .fetch_one(pool)
            .await?,
        );

        Ok(post)
    }

    pub async fn get_with_uuid(uuid: Uuid, pool: &SqlitePool) -> Result<Post, sqlx::Error> {
        let post = Post::from(
            sqlx::query_as!(
                RawPost,
                r#"
            SELECT 
                uuid as "uuid!: uuid::Uuid", 
                draft as "draft!: bool", 
                title, 
                date as "date!: chrono::NaiveDate", 
                updated as "updated!: chrono::NaiveDate", 
                slug, 
                tags,
                content 
            FROM posts
            WHERE uuid=$1
            "#,
                uuid
            )
            .fetch_one(pool)
            .await?,
        );

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
