mod db;
mod templates;
use actix_web::{
    get,
    http::{header::ContentType, StatusCode},
    post, test, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use db::Post;
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
        "<h1>{title}</h1><h2>{date}<h2><p>{content}</p>",
        title = post.title,
        date = post.date,
        content = post.content
    )
}

#[get("/blog/{date}/{slug}")]
async fn render_blog_post(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let posts = db::get_posts().expect("failed to get posts");
    let test_post = &posts[0];
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body(generate_blog_post(test_post.to_owned())))
}

#[get("/contact")]
async fn contact(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body(render_static_html(CONTACT)))
}

#[derive(Deserialize)]
struct Register {
    username: String,
    country: String,
}
async fn register(form: web::Json<Register>) -> impl Responder {
    format!("hello {} from {}", form.username, form.country)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(contact)
            .service(blog)
            .service(render_blog_post)
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .route("/register", web::post().to(register))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
