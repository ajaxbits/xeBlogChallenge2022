use crate::{model::Post, PageCtx};
use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    web, HttpResponse,
};
use serde::Serialize;
use sqlx::SqlitePool;
use tinytemplate::TinyTemplate;

async fn blog(
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
    tt.add_template("blog_list", BLOG_INDEX)
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

fn generate_blog_post(post: Post) -> String {
    let mut tt = tinytemplate::TinyTemplate::new();
    tt.add_template(&post.title, BLOG_POST)
        .expect("could not add the blog_post template");
    tt.render(&post.title, &post).expect(&format!(
        "failed to render blog_post template for {}",
        &post.title
    ))
}

async fn render_blog_post(
    db: web::Data<SqlitePool>,
    base_tt: web::Data<TinyTemplate<'_>>,
    path: web::Path<(String, String)>,
) -> actix_web::Result<HttpResponse> {
    let (date, slug) = path.into_inner();
    let date = chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(error::ErrorInternalServerError)?;
    let post = Post::get_with_dateslug(date, slug, &db)
        .await
        .map_err(error::ErrorNotFound)?;

    let ctx = PageCtx {
        content: generate_blog_post(post.clone()),
        title: post.title,
    };
    let post: String = base_tt
        .render("base", &ctx)
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(post))
}

pub fn blog_config(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(blog))
        .service(web::resource("/{date}/{slug}").route(web::get().to(render_blog_post)));
}

static BLOG_POST: &str = include_str!("../templates/blog_post.html");
pub static BLOG_INDEX: &str = include_str!("../templates/blog.html");
