use actix_identity::Identity;
use actix_web::HttpResponse;

pub async fn login(id: Identity) -> HttpResponse {
    // remember identity
    id.remember("User1".to_owned());
    HttpResponse::Ok().finish()
}

pub async fn logout(id: Identity) -> HttpResponse {
    // remove identity
    id.forget();
    HttpResponse::Ok().finish()
}
