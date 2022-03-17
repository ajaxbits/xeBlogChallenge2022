use crate::{
    auth::{login, logout},
    model::Post,
    PageCtx,
};
use actix_web::{
    error::{self, ErrorInternalServerError},
    http::{self, header::ContentType, StatusCode},
    web, HttpResponse, ResponseError,
};
use actix_web_httpauth::extractors::basic::Config;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::fmt;
use std::str::FromStr;
use tinytemplate::TinyTemplate;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum AdminFunction {
    Add,
    Edit,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum AdminFunctionError {
    NotFound,
}

impl FromStr for AdminFunction {
    type Err = AdminFunctionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "add" => Ok(AdminFunction::Add),
            "edit" => Ok(AdminFunction::Edit),
            _ => Err(AdminFunctionError::NotFound),
        }
    }
}
impl fmt::Display for AdminFunctionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not find the admin path specified.")
    }
}
impl ResponseError for AdminFunctionError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        match self {
            AdminFunctionError::NotFound => HttpResponse::NotFound().finish(),
        }
    }
}

#[derive(Serialize)]
struct LoginForm {
    name: String,
    password: String,
}

#[derive(Serialize)]
struct AdminCtx {
    edit: Option<Post>,
}

#[derive(Deserialize, Debug, Clone)]
struct ComplexPath {
    formmethod: String,
    date: Option<chrono::NaiveDate>,
    slug: Option<String>,
}

/// Adds a post to the post database
async fn add_post(
    params: web::Form<Post>,
    db: web::Data<SqlitePool>,
) -> actix_web::Result<HttpResponse> {
    let new_post = params.into_inner();
    Post::insert(new_post, &db)
        .await
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().finish())
}

/// Edits an existing post in the database, deleting and replacing
async fn edit_post(
    params: web::Form<Post>,
    db: web::Data<SqlitePool>,
) -> actix_web::Result<HttpResponse> {
    let form_fields = params.into_inner();
    let old_post = Post::get_with_uuid(form_fields.uuid, &db)
        .await
        .map_err(ErrorInternalServerError)?;
    let new_post = form_fields;
    Post::update(old_post, new_post, &db)
        .await
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().finish())
}

async fn delete_post(
    params: web::Path<Uuid>,
    db: web::Data<SqlitePool>,
) -> actix_web::Result<HttpResponse> {
    let old_post = Post::get_with_uuid(params.into_inner(), &db)
        .await
        .map_err(ErrorInternalServerError)?;
    Post::delete(old_post, &db)
        .await
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Found()
        .append_header((http::header::LOCATION, "/admin"))
        .finish())
}

async fn form_get(
    path: web::Path<ComplexPath>,
    base_tt: web::Data<TinyTemplate<'_>>,
    db: web::Data<SqlitePool>,
) -> actix_web::Result<HttpResponse> {
    let path = path.into_inner();
    let formmethod = AdminFunction::from_str(&path.formmethod).map_err(error::ErrorNotFound)?;

    let mut tt = TinyTemplate::new();
    tt.add_template("admin", ADMIN_FORM)
        .expect("failed to add admin template");

    let body = match formmethod {
        AdminFunction::Add => tt
            .render("admin", &AdminCtx { edit: None })
            .map_err(error::ErrorNotFound)?,
        AdminFunction::Edit => {
            let date = path.date;
            let slug = path.slug;
            let post = Post::get_with_dateslug(date.unwrap(), slug.unwrap(), &db)
                .await
                .map_err(error::ErrorNotFound)?;
            tt.render("admin", &AdminCtx { edit: Some(post) })
                .map_err(error::ErrorNotFound)?
        }
    };

    let ctx = PageCtx {
        title: "admin".to_string(),
        content: body,
    };
    let body = base_tt
        .render("base", &ctx)
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(body))
}

async fn form_post(
    path: web::Path<ComplexPath>,
    params: web::Form<Post>,
    db: web::Data<SqlitePool>,
) -> actix_web::Result<HttpResponse> {
    let path = path.into_inner();
    let formmethod = AdminFunction::from_str(&path.formmethod).map_err(error::ErrorNotFound);
    let formmethod = formmethod?;
    match formmethod {
        AdminFunction::Add => {
            add_post(params, db)
                .await
                .map_err(ErrorInternalServerError)?;
        }
        AdminFunction::Edit => {
            edit_post(params, db)
                .await
                .map_err(ErrorInternalServerError)?;
        }
    }
    Ok(HttpResponse::Found()
        .append_header((http::header::LOCATION, "/admin"))
        .finish())
}

async fn list(
    db: web::Data<SqlitePool>,
    base_tt: web::Data<TinyTemplate<'_>>,
) -> actix_web::Result<HttpResponse> {
    #[derive(Serialize)]
    struct PostListContext {
        post_list: Vec<Post>,
    }
    let post_list = Post::all(&db).await.map_err(ErrorInternalServerError)?;

    let mut tt = TinyTemplate::new();
    tt.add_template("blog_list", ADMIN_LIST)
        .map_err(ErrorInternalServerError)?;
    let body = tt
        .render("blog_list", &PostListContext { post_list })
        .expect("could not put the blog post list into the blog post index page");
    let ctx = PageCtx {
        content: body,
        title: "blog".to_string(),
    };
    let body = base_tt
        .render("base", &ctx)
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(body))
}

pub fn admin_config(cfg: &mut web::ServiceConfig) {
    cfg.app_data(Config::default().realm("Restricted area"))
        .route("", web::get().to(list))
        .route("/login", web::post().to(login))
        .route("/logout", web::post().to(logout))
        .service(web::resource("/delete/{uuid}").route(web::get().to(delete_post)))
        .service(
            web::resource(["/{formmethod}", "/{formmethod}/{date}/{slug}"])
                .route(web::get().to(form_get))
                .route(web::post().to(form_post)),
        );
}

static ADMIN_LIST: &str = include_str!("../templates/admin_list.html");
static ADMIN_FORM: &str = include_str!("../templates/admin_add.html");
