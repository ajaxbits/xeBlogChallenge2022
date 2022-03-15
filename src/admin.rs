use crate::{
    auth::{login, logout},
    model::{self, Post},
    PageCtx,
};
use actix_web::{
    dev::ServiceRequest,
    error,
    http::{header::ContentType, StatusCode},
    web::{self, Data, PathConfig},
    HttpResponse, ResponseError,
};
use actix_web_httpauth::extractors::basic::{BasicAuth, Config};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::str::FromStr;
use std::{fmt, sync::Mutex};
use tinytemplate::TinyTemplate;

#[derive(Serialize)]
struct LoginForm {
    name: String,
    password: String,
}

#[derive(Serialize)]
struct AdminCtx {
    edit: Option<Post>,
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
    // delete post
    // add post
    todo!()
}

async fn list_posts(
    post_mutex: Data<Mutex<Post>>,
    db: web::Data<SqlitePool>,
    base_tt: web::Data<TinyTemplate<'_>>,
) -> actix_web::Result<HttpResponse> {
    #[derive(Serialize)]
    struct PostListContext {
        post_list: Vec<Post>,
    }

    let post_list = Post::all(&db)
        .await
        .map_err(error::ErrorInternalServerError)?;

    let mut tt = TinyTemplate::new();
    tt.add_template("blog_list", ADMIN_LIST)
        .map_err(error::ErrorInternalServerError)?;
    let body = tt
        .render("blog_list", &PostListContext { post_list })
        .expect("could not put the blog post list into the blog post index page");
    let ctx = PageCtx {
        content: body,
        title: "blog".to_string(),
    };
    let body = base_tt
        .render("base", &ctx)
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(body))
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum FormMethod {
    Add,
    Edit,
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

#[derive(Deserialize, Debug)]
struct ComplexPath {
    formmethod: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slug: Option<String>,
}

/// Serves the "Add Page" form
async fn form(
    path: web::Path<ComplexPath>,
    base_tt: web::Data<TinyTemplate<'_>>,
    db: web::Data<SqlitePool>,
) -> actix_web::Result<HttpResponse> {
    println!("{:#?}", path);

    let path = path.into_inner();

    let formmethod = FormMethod::from_str(&path.formmethod).map_err(error::ErrorNotFound)?;

    let mut tt = TinyTemplate::new();
    tt.add_template("admin", ADMIN_FORM)
        .expect("failed to add admin template");

    let body = match formmethod {
        FormMethod::Add => tt
            .render("admin", &AdminCtx { edit: None })
            .map_err(error::ErrorNotFound)?,
        FormMethod::Edit => {
            let date = path.date;
            let slug = path.slug;
            let post = Post::get(date.unwrap(), slug.unwrap(), &db)
                .await
                .map_err(error::ErrorNotFound)?;
            tt.render("admin", &AdminCtx { edit: Some(post) })
                .map_err(error::ErrorNotFound)?
        }
    };

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
    path: web::Path<ComplexPath>,
    params: web::Form<Post>,
    db: web::Data<SqlitePool>,
) -> actix_web::Result<HttpResponse> {
    println!("{:#?}", path);

    let formmethod =
        FormMethod::from_str(&path.into_inner().formmethod).map_err(error::ErrorNotFound);
    println!("{:?}", formmethod);
    let formmethod = formmethod?;
    match formmethod {
        FormMethod::Add => add_post(params, db)
            .await
            .map_err(error::ErrorInternalServerError)?,
        FormMethod::Edit => edit_post(params, db)
            .await
            .map_err(error::ErrorInternalServerError)?,
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
    let post_mutex = Data::new(Mutex::new(Post::new(
        "title",
        "2000-01-01",
        "slug",
        "content",
    )));
    cfg.app_data(Config::default().realm("Restricted area"))
        .route("", web::get().to(admin))
        .route("/login", web::post().to(login))
        .route("/logout", web::post().to(logout))
        .app_data(post_mutex.clone())
        .service(web::resource("/list").route(web::get().to(list_posts)))
        // .service(web::resource("/edit-post").route(web::get().to(edit_handler)))
        .service(
            web::resource(["/{formmethod}", "/{formmethod}/{date}/{slug}"])
                .route(web::get().to(form))
                .route(web::post().to(form_post)),
        );
}

static ADMIN_INDEX: &str = include_str!("../templates/admin.html");
static ADMIN_LIST: &str = include_str!("../templates/admin_list.html");
static ADMIN_FORM: &str = include_str!("../templates/admin_add.html");
