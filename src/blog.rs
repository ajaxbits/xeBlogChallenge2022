use crate::db::{self, Pool, Post};
use actix_web::{
    get,
    http::{header::ContentType, StatusCode},
    web, HttpResponse,
};

async fn blog(db: web::Data<Pool>) -> actix_web::Result<HttpResponse> {
    let posts = db::execute_get_post_index(&db).await;
    let mut list = Vec::new();
    list.push("<ul>");
    list.push("</ul>");
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(format!(
            "<h1>This is the blog index</h1><ul>{:?}</ul>",
            posts
        )))
}

fn generate_blog_post(post: Post) -> String {
    format!(
        "<html><h1>{title}</h1><h2>{date}<h2><p>{content}</p></html>",
        title = post.title,
        date = post.date,
        content = post.content
    )
}

async fn render_blog_post(db: web::Data<Pool>, path: web::Path<(String, String)>) -> HttpResponse {
    let (date, slug) = path.into_inner();
    let post = db::execute_get_post(&db, date, slug).await;
    match post {
        Ok(post) => HttpResponse::build(StatusCode::OK)
            .content_type(ContentType::html())
            .body(generate_blog_post(post.to_owned())),
        Err(_) => HttpResponse::build(StatusCode::NOT_FOUND).finish(),
    }
}

pub fn blog_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("").route(web::get().to(blog)))
        .service(web::resource("/{date}/{slug}").route(web::get().to(render_blog_post)));
}
