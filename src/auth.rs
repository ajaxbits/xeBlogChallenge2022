use actix_identity::Identity;
use actix_web::{dev::ServiceRequest, HttpResponse};
use actix_web_httpauth::extractors::basic::BasicAuth;

pub async fn admin_validator(
    req: ServiceRequest,
    creds: BasicAuth,
) -> Result<ServiceRequest, actix_web::Error> {
    println!("{:#?}", creds.user_id());
    println!("{:#?}", creds.password());
    Ok(req)
}

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
