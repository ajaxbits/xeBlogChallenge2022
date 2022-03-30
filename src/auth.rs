use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::{
    basic::{BasicAuth, Config},
    bearer::BearerAuth,
    AuthenticationError,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

const USER: &str = "alex";
const PASS: &str = "pass";
const TOKEN: &str = "a-secure-token";

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct User {
    pub uuid: Uuid,
    pub user: String,
    pub pass: String,
}

#[derive(Serialize, Clone, Deserialize, Debug)]
struct SqlUser {
    uuid: Vec<u8>,
    user: String,
    pass: String,
}

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    pw: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

impl From<SqlUser> for User {
    fn from(raw_user: SqlUser) -> Self {
        User {
            uuid: Uuid::from_slice(&raw_user.uuid).expect("could not parse uuid"),
            user: raw_user.user,
            pass: raw_user.pass,
        }
    }
}

impl User {
    pub async fn fetch_all(pool: &SqlitePool) -> Result<Vec<User>, sqlx::Error> {
        let users: Vec<User> = sqlx::query_as!(
            SqlUser,
            r#"
            SELECT uuid, user, pass
            FROM users
            "#
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|u| User::from(u))
        .collect();

        Ok(users)
    }

    pub async fn add(new_user: User, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO users
            VALUES ($1,$2,$3)
            "#,
            new_user.uuid,
            new_user.user,
            new_user.pass
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn delete(user: User, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM users
            WHERE uuid=$1
            "#,
            user.uuid
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    // pub async fn get_with_uuid(uuid: Uuid, pool: &SqlitePool) -> Result<User, sqlx::Error> {
    //     let user: User = sqlx::query_as!(
    //         SqlUser,
    //         r#"
    //         SELECT uuid, user, pass
    //         FROM users
    //         WHERE uuid=$1
    //         "#,
    //         uuid
    //     )
    //     .fetch_one(pool)
    //     .await?
    //     .into();
    //
    //     Ok(user)
    // }
}

async fn bearer_auth_validator(
    req: ServiceRequest,
    creds: BearerAuth,
) -> actix_web::Result<ServiceRequest> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);

    let token = creds.token();

    match token.eq(TOKEN) {
        true => Ok(req),
        false => Err(AuthenticationError::from(config).into()),
    }
}

pub async fn admin_validator(
    req: ServiceRequest,
    creds: BasicAuth,
) -> Result<ServiceRequest, actix_web::Error> {
    // TODO I don't understand this
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);

    let user = creds.user_id();
    // checks for an existing password, returns a result that is porpogated
    // to the main fn, and returns proper error if no password is provided
    let pass = creds
        .password()
        .ok_or::<actix_web::Error>(AuthenticationError::from(config.clone()).into())?;

    match user.eq(USER) && pass.eq(PASS) {
        true => Ok(req),
        false => Err(AuthenticationError::from(config).into()),
    }
}
