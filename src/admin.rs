use crate::PageCtx;
use actix_identity::Identity;
use actix_web::{
    dev::ServiceRequest,
    http::{header::ContentType, StatusCode},
    web, HttpResponse,
};
use actix_web_httpauth::extractors::basic::{BasicAuth, Config};
use tinytemplate::TinyTemplate;

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

async fn login(id: Identity) -> HttpResponse {
    // remember identity
    id.remember("User1".to_owned());
    HttpResponse::Ok().finish()
}

async fn logout(id: Identity) -> HttpResponse {
    // remove identity
    id.forget();
    HttpResponse::Ok().finish()
}

pub fn admin_config(cfg: &mut web::ServiceConfig) {
    cfg.app_data(Config::default().realm("Restricted area"))
        .route("", web::get().to(admin))
        .route("login", web::post().to(login))
        .route("logout", web::post().to(logout));
}

static ADMIN_INDEX: &str = include_str!("../templates/admin.html");
