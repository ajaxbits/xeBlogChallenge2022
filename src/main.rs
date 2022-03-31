mod admin;
mod auth;
mod blog;
mod db;
mod model;
mod templates;
use actix_web::{
    dev::Service as _,
    get,
    http::{
        header::{ContentType, HeaderName, CONTENT_TYPE, FORWARDED},
        StatusCode,
    },
    middleware, services, web, App, HttpMessage, HttpResponse, HttpServer,
};
use actix_web_httpauth::middleware::HttpAuthentication;
use auth::admin_validator;
use axum::http::HeaderValue;
use serde::Serialize;
use sqlx::{Pool, Sqlite};
use std::{fs, path::Path};
use tinytemplate::TinyTemplate;

const INDEX: &str = "static/index.html";
const CONTACT: &str = "static/contact.html";

#[derive(Serialize, Debug)]
struct PageCtx {
    title: String,
    content: String,
}

fn render_static_html(file_name: &str) -> String {
    let path = Path::new(file_name);
    let body: String =
        fs::read_to_string(path).expect(&format!("can't read {} to string", file_name));
    body
}

#[get("/")]
async fn index(tt: web::Data<TinyTemplate<'_>>) -> HttpResponse {
    HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(
            tt.render(
                "base",
                &PageCtx {
                    title: String::new(),
                    content: render_static_html(INDEX),
                },
            )
            .expect("failed to render base template"),
        )
}

#[get("/contact")]
async fn contact(tt: web::Data<TinyTemplate<'_>>) -> HttpResponse {
    HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(
            tt.render(
                "base",
                &PageCtx {
                    title: "Contact".to_string(),
                    content: render_static_html(CONTACT),
                },
            )
            .expect("failed to render base template"),
        )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let pool: Pool<Sqlite> = db::init_pool("sqlite://posts.db")
        .await
        .expect("had trouble creating the sqlite pool...");

    HttpServer::new(move || {
        let mut tt = tinytemplate::TinyTemplate::new();
        tt.add_template("base", BASE)
            .expect("failed to add base template");
        let admin_auth = HttpAuthentication::basic(admin_validator);

        App::new()
            .wrap(actix_web::middleware::Compress::default())
            .wrap_fn(|req, srv| {
                let fut = srv.call(req);
                async {
                    let res = fut.await?;
                    println!("{:?}", res.headers());
                    let headers = res.headers().iter().collect::<Vec<_>>();
                    println!("{:?}", headers);
                    Ok(res)
                }
            })
            .app_data(web::Data::new(tt))
            .app_data(web::Data::new(pool.clone()))
            .service(services![index, contact])
            .service(web::scope("/blog").configure(blog::blog_config))
            .service(
                web::scope("/admin")
                    .wrap(admin_auth)
                    .app_data(
                        actix_web_httpauth::extractors::basic::Config::default()
                            .realm("Restricted area"),
                    )
                    .configure(admin::admin_config),
            )
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

static BASE: &str = include_str!("../templates/base.html");
static ERROR: &str = include_str!("../templates/error.html");
