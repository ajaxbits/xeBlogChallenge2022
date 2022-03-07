use std::{
    fs::{self, File},
    io::Read,
    path::Path,
};

use actix_web::{get, http::header::ContentType, App, HttpRequest, HttpResponse, HttpServer};
use axum::http::StatusCode;

mod datatypes;
mod templates;

const INDEX: &str = "static/index.html";
const CONTACT: &str = "static/contact.html";

fn render_static_html(file_name: &str) -> String {
    let path = Path::new(file_name);
    let body: String =
        fs::read_to_string(path).expect(&format!("can't read {} to string", file_name));
    body
}
#[get("/")]
async fn index(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body(render_static_html(INDEX)))
}

#[get("/blog/{date}/{slug}")]
async fn blog(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    unimplemented!()
}

#[get("/contact")]
async fn contact(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body(render_static_html(CONTACT)))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(contact)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
