use actix_web::{get, http::header::ContentType, App, HttpRequest, HttpResponse, HttpServer};
use axum::http::StatusCode;

mod datatypes;
mod templates;

#[get("/")]
async fn index(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    println!("{:?}", req);
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body("<h1>Hello, world!</h1>"))
}

#[get("/blog/{date}/{slug}")]
async fn blog(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    unimplemented!()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
