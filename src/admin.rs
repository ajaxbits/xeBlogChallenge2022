use crate::{
    auth::{login, logout},
    PageCtx,
};
use actix_web::{
    dev::ServiceRequest,
    http::{header::ContentType, StatusCode},
    web, HttpResponse, HttpResponseBuilder,
};
use actix_web_httpauth::extractors::basic::{BasicAuth, Config};
use serde::Serialize;
use tinytemplate::TinyTemplate;

#[derive(Serialize)]
struct LoginForm {
    name: String,
    password: String,
}

pub async fn admin_validator(
    req: ServiceRequest,
    creds: BasicAuth,
) -> Result<ServiceRequest, actix_web::Error> {
    println!("{:#?}", creds.user_id());
    println!("{:#?}", creds.password());
    Ok(req)
}

async fn admin(
    id: actix_identity::Identity,
    base_tt: web::Data<TinyTemplate<'_>>,
) -> actix_web::Result<HttpResponse> {
    let ctx = PageCtx {
        title: "admin".to_string(),
        content: ADMIN_INDEX.to_string(),
    };
    let body = base_tt.render("base", &ctx).unwrap();

    if let Some(id) = id.identity() {
        println!("{:#?}", id);
        Ok(HttpResponse::build(StatusCode::OK)
            .content_type(ContentType::html())
            .body("you logged in!"))
    } else {
        Ok(HttpResponse::build(StatusCode::OK)
            .content_type(ContentType::html())
            .body(body))
    }
}

/// Adds a post to the post database
async fn add_post() -> HttpResponseBuilder {
    HttpResponse::Ok()
}

/// Serves the "Add Page" form
async fn add_page() -> HttpResponseBuilder {
    HttpResponse::Ok()
}

async fn edit_post() -> HttpResponse {
    todo!()
}

async fn edit_page() -> HttpResponse {
    todo!()
}

async fn list() -> HttpResponse {
    todo!()
}

pub fn admin_config(cfg: &mut web::ServiceConfig) {
    cfg.app_data(Config::default().realm("Restricted area"))
        .route("", web::get().to(admin))
        .route("/login", web::post().to(login))
        .route("/logout", web::post().to(logout))
        .service(
            web::resource("/add")
                .route(web::post().to(add_post))
                .route(web::get().to(add_page)),
        )
        .service(
            web::resource("/edit")
                .route(web::post().to(edit_post))
                .route(web::get().to(edit_page)),
        )
        .route("logout", web::post().to(logout));
}

static ADMIN_INDEX: &str = include_str!("../templates/admin.html");
