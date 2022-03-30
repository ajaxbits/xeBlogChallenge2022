use crate::{model::Post, PageCtx};
use actix_web::{
    error::{self, ErrorInternalServerError},
    http::{self, header::ContentType, StatusCode},
    web, HttpResponse,
};
use actix_web_httpauth::extractors::basic::Config;
use serde::Serialize;
use sqlx::SqlitePool;
use tinytemplate::TinyTemplate;
use uuid::Uuid;

#[derive(Serialize)]
struct AdminCtx {
    edit: Option<Post>,
}

/// Adds a post to the post database
async fn add_post(
    params: web::Form<Post>,
    db: web::Data<SqlitePool>,
) -> actix_web::Result<HttpResponse> {
    let new_post: Post = params.into_inner();
    Post::insert(new_post, &db)
        .await
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Found()
        .append_header((http::header::LOCATION, "/admin"))
        .finish())
}

/// Edits an existing post in the database, deleting and replacing
async fn edit_post(
    path: web::Path<String>,
    params: web::Form<Post>,
    db: web::Data<SqlitePool>,
) -> actix_web::Result<HttpResponse> {
    let og_uuid: String = path.into_inner();
    let og_uuid: Uuid = Uuid::parse_str(&og_uuid).map_err(ErrorInternalServerError)?;
    let old_post = Post::get_with_uuid(og_uuid, &db)
        .await
        .map_err(ErrorInternalServerError)?;
    let new_post = params.into_inner();
    Post::update(old_post, new_post, &db)
        .await
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Found()
        .append_header((http::header::LOCATION, "/admin"))
        .finish())
}

/// Deletes an existing post in the database by uuid
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

/// Renders the form to add a post
async fn add_page(base_tt: web::Data<TinyTemplate<'_>>) -> actix_web::Result<HttpResponse> {
    let mut tt = TinyTemplate::new();
    tt.add_template("admin", ADMIN_FORM)
        .expect("failed to add admin template");

    let body = tt
        .render("admin", &AdminCtx { edit: None })
        .map_err(error::ErrorNotFound)?;

    let ctx = PageCtx {
        title: "add_post".to_string(),
        content: body,
    };
    let body = base_tt
        .render("base", &ctx)
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(body))
}

/// Renders the form to edit a post
async fn edit_page(
    path: web::Path<String>,
    base_tt: web::Data<TinyTemplate<'_>>,
    db: web::Data<SqlitePool>,
) -> actix_web::Result<HttpResponse> {
    let uuid: String = path.into_inner();
    let uuid: Uuid = Uuid::parse_str(&uuid).map_err(ErrorInternalServerError)?;
    let mut tt = TinyTemplate::new();
    tt.add_template("admin", ADMIN_FORM)
        .expect("failed to add admin template");

    let body = {
        let post = Post::get_with_uuid(uuid, &db)
            .await
            .map_err(error::ErrorNotFound)?;
        tt.render("admin", &AdminCtx { edit: Some(post) })
            .map_err(error::ErrorNotFound)?
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

/// Renders the list of posts for the admin to view, edit, and delete
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
        .service(web::resource("/delete/{uuid}").route(web::get().to(delete_post)))
        .service(
            web::resource("/add")
                .route(web::get().to(add_page))
                .route(web::post().to(add_post)),
        )
        .service(
            web::resource("/edit/{uuid}")
                .route(web::get().to(edit_page))
                .route(web::post().to(edit_post)),
        );
}

static ADMIN_LIST: &str = include_str!("../templates/admin_list.html");
static ADMIN_FORM: &str = include_str!("../templates/admin_add.html");
