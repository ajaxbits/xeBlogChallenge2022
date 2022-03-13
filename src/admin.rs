use actix_web::{
    http::{header::ContentType, StatusCode},
    web, HttpResponse,
};
use serde_json::json;
use tinytemplate::TinyTemplate;

async fn admin(base_tt: web::Data<TinyTemplate<'_>>) -> actix_web::Result<HttpResponse> {
    let ctx = json!({
        "content": ADMIN_INDEX,
        "title": "blog"
    });
    let body = base_tt.render("base", &ctx).unwrap();

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(body))
}
pub fn admin_config(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(admin));
}

static ADMIN_INDEX: &str = include_str!("../templates/admin.html");
