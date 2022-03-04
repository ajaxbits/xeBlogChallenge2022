use actix_web::{
    get, http::header::ContentType, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use axum::{http::StatusCode, service, Router};
use std::{convert::Infallible, fs, net::SocketAddr, path::Path, thread, time::Duration};
use tower_http::services::ServeDir;

mod datatypes;
mod templates;
const CONTENT_DIR: &str = "content";
const PUBLIC_DIR: &str = "public";

#[get("/")]
async fn index(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    println!("{:?}", req);
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body("<h1>Hello, world!</h1>"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
