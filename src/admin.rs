use crate::db::{self, Pool, Post};
use actix_files::Files;
use actix_web::{
    get,
    http::{header::ContentType, StatusCode},
    post, test, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use r2d2_sqlite::SqliteConnectionManager;
use serde::Deserialize;
use std::{fs, path::Path};

async fn admin(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("admin page")
}

pub fn admin_config(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(admin));
}
