mod admin;
mod blog;
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
use tinytemplate::TinyTemplate;

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
        .content_type(ContentType::html())
        .body(render_static_html(INDEX))
}

#[get("/contact")]
async fn contact(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(render_static_html(CONTACT)))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let manager = SqliteConnectionManager::file("posts.db");
    let pool = Pool::new(manager).unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Compress::default())
            .wrap(actix_web::middleware::Logger::default())
            .service(index)
            .service(contact)
            .service(
                web::scope("/blog")
                    .configure(blog::blog_config)
                    .app_data(web::Data::new(pool.clone())),
            )
            .service(web::scope("/admin").configure(admin::admin_config))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

static ERROR: &str = include_str!("../templates/error.html");
// static INDEX: &str = include_str!("templates/index.html");
