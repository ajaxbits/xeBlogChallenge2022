use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::{
    basic::{BasicAuth, Config},
    AuthenticationError,
};

const USER: &str = "alex";
const PASS: &str = "pass";

pub async fn admin_validator(
    req: ServiceRequest,
    creds: BasicAuth,
) -> Result<ServiceRequest, actix_web::Error> {
    // TODO I don't understand this
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);

    let user = creds.user_id();
    // checks for an existing password, and returns proper error
    let pass = creds
        .password()
        .ok_or::<actix_web::Error>(AuthenticationError::from(config.clone()).into())?;

    match user.eq(USER) && pass.eq(PASS) {
        true => Ok(req),
        false => Err(AuthenticationError::from(config).into()),
    }
}
