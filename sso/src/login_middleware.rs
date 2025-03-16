use crate::errors::login::AuthError;
use crate::AppState;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::header::{HeaderValue, CACHE_CONTROL};
use actix_web::middleware::Next;
use actix_web::{web, Error};
use std::rc::Rc;

fn authenticate(req: &ServiceRequest) -> Result<(), AuthError> {
    let state: &web::Data<AppState> = req.app_data().unwrap();

    let cookie = match req.cookie("token") {
        None => return Err(AuthError::TokenNotExist),
        Some(cookie) => cookie
    };
    let user = state.auth.authenticate(cookie.value())?;
    *state.user.lock().unwrap() = Some(user);
    Ok(())
}

pub async fn login_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let next = Rc::new(next);

    let excluded_path = [
        "/login",
        "/",
        "/register",
    ];

    let public = excluded_path.contains(&req.path());

    match (authenticate(&req), public) {
        (_, true) => (),
        (Ok(()), _) => (),
        (Err(e), _) => return Err(Error::from(e))
    }

    let mut res = next.call(req).await?;
    res.headers_mut().insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));

    Ok(res)
}