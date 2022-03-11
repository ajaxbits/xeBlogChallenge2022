mod db;
mod templates;
use actix_files::Files;
use actix_web::{
    get,
    http::{header::ContentType, StatusCode},
    post, test, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use db::{Pool, Post};
use r2d2_sqlite::SqliteConnectionManager;
use serde::Deserialize;
use std::{fs, path::Path};

const INDEX: &str = "static/index.html";
const CONTACT: &str = "static/contact.html";

fn render_static_html(file_name: &str) -> String {
    let path = Path::new(file_name);
    let body: String =
        fs::read_to_string(path).expect(&format!("can't read {} to string", file_name));
    body
}
#[get("/")]
async fn index(req: HttpRequest) -> HttpResponse {
    HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body(render_static_html(INDEX))
}

#[get("/admin")]
async fn admin(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    unimplemented!()
}

#[get("/blog")]
async fn blog() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body("<h1>This is the blog index</h1>"))
}

fn generate_blog_post(post: Post) -> String {
    format!(
        "<html><h1>{title}</h1><h2>{date}<h2><p>{content}</p></html>",
        title = post.title,
        date = post.date,
        content = post.content
    )
}

#[get("/blog/{date}/{slug}")]
async fn render_blog_post(db: web::Data<Pool>, path: web::Path<(String, String)>) -> HttpResponse {
    let (date, slug) = path.into_inner();
    let post = db::execute(&db, date, slug).await;
    match post {
        Ok(post) => HttpResponse::build(StatusCode::OK)
            .content_type(ContentType::html())
            .body(generate_blog_post(post.to_owned())),
        Err(_) => HttpResponse::build(StatusCode::NOT_FOUND).finish(),
    }
}

#[get("/contact")]
async fn contact(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body(render_static_html(CONTACT)))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Connect to SQLite
    let manager = SqliteConnectionManager::file("posts.db");
    let pool = Pool::new(manager).unwrap();

    // Serve app
    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            // store db pool as Data object
            .app_data(web::Data::new(pool.clone()))
            .service(index)
            .service(contact)
            .service(blog)
            .service(render_blog_post)
            .service(Files::new("/static", "static").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
