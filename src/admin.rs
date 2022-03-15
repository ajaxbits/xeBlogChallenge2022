use crate::{
    auth::{login, logout},
    model::Post,
    PageCtx,
};
use actix_web::{
    dev::ServiceRequest,
    error,
    http::{header::ContentType, StatusCode},
    web, HttpResponse, ResponseError,
};
use actix_web_httpauth::extractors::basic::{BasicAuth, Config};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::fmt;
use std::str::FromStr;
use tinytemplate::TinyTemplate;

#[derive(Serialize)]
struct LoginForm {
    name: String,
    password: String,
}

#[derive(Serialize)]
struct AdminCtx {
    add: bool,
}

pub async fn admin_validator(
    req: ServiceRequest,
    creds: BasicAuth,
) -> Result<ServiceRequest, actix_web::Error> {
    println!("{:#?}", creds.user_id());
    println!("{:#?}", creds.password());
    Ok(req)
}

/// Adds a post to the post database
async fn add_post(
    params: web::Form<Post>,
    db: web::Data<SqlitePool>,
) -> actix_web::Result<HttpResponse> {
    let new_post = params.into_inner();
    Post::insert(new_post, &db)
        .await
        .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().finish())
}

/// Edits an existing post in the database, deleting and replacing
async fn edit_post(
    params: web::Form<Post>,
    db: web::Data<SqlitePool>,
) -> actix_web::Result<HttpResponse> {
    todo!()
}

async fn list_posts() -> HttpResponse {
    todo!()
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum FormMethod {
    Add,
    Edit,
    List,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum FormMethodError {
    NotFound,
}

impl FromStr for FormMethod {
    type Err = FormMethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "add" => Ok(FormMethod::Add),
            "edit" => Ok(FormMethod::Edit),
            "list" => Ok(FormMethod::List),
            _ => Err(FormMethodError::NotFound),
        }
    }
}

impl fmt::Display for FormMethodError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not find the admin path specified.")
    }
}

impl ResponseError for FormMethodError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            FormMethodError::NotFound => HttpResponse::NotFound().finish(),
        }
    }
}

/// Serves the "Add Page" form
async fn form(
    path: web::Path<String>,
    base_tt: web::Data<TinyTemplate<'_>>,
) -> actix_web::Result<HttpResponse> {
    let formmethod = FormMethod::from_str(&path.into_inner())?;

    let mut tt = TinyTemplate::new();
    tt.add_template("admin", ADMIN_FORM)
        .expect("failed to add admin template");

    let body = match formmethod {
        FormMethod::Add => tt
            .render("admin", &AdminCtx { add: true })
            .map_err(error::ErrorNotFound),
        FormMethod::Edit => tt
            .render("admin", &AdminCtx { add: false })
            .map_err(error::ErrorNotFound),
        FormMethod::List => todo!(),
    }?;

    let ctx = PageCtx {
        title: "add_page".to_string(),
        content: body,
    };

    let body = base_tt
        .render("base", &ctx)
        .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(body))
}

async fn form_post(
    path: web::Path<String>,
    params: web::Form<Post>,
    db: web::Data<SqlitePool>,
) -> actix_web::Result<HttpResponse> {
    let formmethod = FormMethod::from_str(&path.into_inner()).map_err(error::ErrorNotFound);
    println!("{:?}", formmethod);
    let formmethod = formmethod?;
    match formmethod {
        FormMethod::Add => add_post(params, db)
            .await
            .map_err(error::ErrorInternalServerError)?,
        FormMethod::Edit => edit_post(params, db)
            .await
            .map_err(error::ErrorInternalServerError)?,
        FormMethod::List => todo!(),
    };
    Ok(HttpResponse::Ok().finish())
}

async fn admin(
    id: actix_identity::Identity,
    base_tt: web::Data<TinyTemplate<'_>>,
) -> actix_web::Result<HttpResponse> {
    let ctx = PageCtx {
        title: "admin".to_string(),
        content: ADMIN_INDEX.to_string(),
    };
    let body = base_tt.render("base", &ctx).unwrap();

    if let Some(id) = id.identity() {
        println!("{:#?}", id);
        Ok(HttpResponse::build(StatusCode::OK)
            .content_type(ContentType::html())
            .body("you logged in!"))
    } else {
        Ok(HttpResponse::build(StatusCode::OK)
            .content_type(ContentType::html())
            .body(body))
    }
}

pub fn admin_config(cfg: &mut web::ServiceConfig) {
    cfg.app_data(Config::default().realm("Restricted area"))
        .route("", web::get().to(admin))
        .route("/login", web::post().to(login))
        .route("/logout", web::post().to(logout))
        .service(
            web::resource("/{formmethods}")
                .route(web::get().to(form))
                .route(web::post().to(form_post)),
        );
}

static ADMIN_INDEX: &str = include_str!("../templates/admin.html");
static ADMIN_FORM: &str = include_str!("../templates/admin_add.html");
