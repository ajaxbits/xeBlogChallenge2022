use crate::db::{self, Pool, Post};
use actix_web::{
    get,
    http::{header::ContentType, StatusCode},
    web, HttpResponse,
};
use serde::Serialize;
use serde_json::json;
use tinytemplate::TinyTemplate;

async fn blog(
    db: web::Data<Pool>,
    base_tt: web::Data<TinyTemplate<'_>>,
) -> actix_web::Result<HttpResponse> {
    #[derive(Serialize)]
    struct PostListContext {
        post_list: Vec<Post>,
    }

    let post_list = db::execute_get_post_index(&db)
        .await
        .expect("Failed to get the posts index");

    let mut tt = TinyTemplate::new();
    tt.add_template("blog_list", BLOG_INDEX).unwrap();
    let body = tt
        .render("blog_list", &PostListContext { post_list })
        .expect("could not put the blog post list into the blog post index page");
    let ctx = json!({
        "content": body,
        "title": "blog"
    });
    let body = base_tt.render("base", &ctx).unwrap();

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(body))
}

fn generate_blog_post(post: Post) -> String {
    let mut tt = tinytemplate::TinyTemplate::new();
    tt.add_template(&post.title, BLOG_POST)
        .expect("could not add the blog_post template");
    tt.render(&post.title, &post).expect(&format!(
        "failed to render blog_post template for {}",
        &post.title
    ))
}

async fn render_blog_post(
    db: web::Data<Pool>,
    base_tt: web::Data<TinyTemplate<'_>>,
    path: web::Path<(String, String)>,
) -> HttpResponse {
    // TODO figure out how to get these errors to interact more elegaltly
    let (date, slug) = path.into_inner();
    let post = db::execute_get_post(&db, date, slug).await;
    match post {
        Ok(post) => {
            let ctx = json!({
                "content": generate_blog_post(post.clone()),
                "title": post.title,
            });
            let post: String = base_tt
                .render("base", &ctx)
                .expect("failed to render base template for blog post");

            HttpResponse::build(StatusCode::OK)
                .content_type(ContentType::html())
                .body(post)
        }
        Err(_) => HttpResponse::build(StatusCode::NOT_FOUND).finish(),
    }
}

pub fn blog_config(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(blog))
        .service(web::resource("/{date}/{slug}").route(web::get().to(render_blog_post)));
}

static BLOG_POST: &str = include_str!("../templates/blog_post.html");
static BLOG_INDEX: &str = include_str!("../templates/blog.html");
