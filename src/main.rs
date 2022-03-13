mod admin;
mod blog;
mod db;
mod templates;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{
    get,
    http::{header::ContentType, StatusCode},
    services, web, App, HttpRequest, HttpResponse, HttpServer,
};
use actix_web_httpauth::middleware::HttpAuthentication;
use admin::admin_validator;
use chrono::NaiveDate;
use db::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde::Serialize;
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
async fn index(tt: web::Data<TinyTemplate<'_>>) -> HttpResponse {
    #[derive(Serialize)]
    struct Ctx {
        title: String,
        content: String,
    }

    HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(
            tt.render(
                "base",
                &Ctx {
                    title: String::new(),
                    content: render_static_html(INDEX),
                },
            )
            .expect("filed to render base template"),
        )
}

#[get("/contact")]
async fn contact(tt: web::Data<TinyTemplate<'_>>) -> HttpResponse {
    #[derive(Serialize)]
    struct Ctx {
        title: String,
        content: String,
    }

    HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(
            tt.render(
                "base",
                &Ctx {
                    title: "Contact".to_string(),
                    content: render_static_html(CONTACT),
                },
            )
            .expect("filed to render base template"),
        )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let manager = SqliteConnectionManager::file("posts.db");
    let pool = Pool::new(manager).unwrap();

    HttpServer::new(move || {
        let mut tt = tinytemplate::TinyTemplate::new();
        tt.add_template("base", BASE)
            .expect("failed to add base template");
        let admin_auth = HttpAuthentication::basic(admin_validator);
        let policy = CookieIdentityPolicy::new(&[0; 32])
            .name("auth-cookie")
            .secure(false);

        App::new()
            .wrap(actix_web::middleware::Compress::default())
            .wrap(actix_web::middleware::Logger::default())
            .app_data(web::Data::new(tt))
            .service(services![index, contact])
            .service(
                web::scope("/blog")
                    .configure(blog::blog_config)
                    .app_data(web::Data::new(pool.clone())),
            )
            .service(
                web::scope("/admin")
                    // .wrap(admin_auth)
                    .wrap(IdentityService::new(policy))
                    .configure(admin::admin_config),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

static BASE: &str = include_str!("../templates/base.html");
#[derive(Serialize, Debug)]
struct PageCtx {
    title: String,
    content: String,
}
static ERROR: &str = include_str!("../templates/error.html");
// static INDEX: &str = include_str!("templates/index.html");
