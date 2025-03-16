use std::rc::Rc;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{web, Error};
use actix_web::middleware::Next;
use crate::AppState;
use crate::errors::login::AuthError;

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

    let state: &web::Data<AppState> = req.app_data().unwrap();

    let cookie = match req.cookie("token") {
        None if excluded_path.contains(&req.path()) => {
            return Ok(next.call(req).await?)
        }
        None => return Err(Error::from(AuthError::TokenNotExist)),
        Some(cookie) => cookie
    };
    let user = match state.auth.authenticate(cookie.value()) {
        Err(_) if excluded_path.contains(&req.path()) => {
            return Ok(next.call(req).await?)
        }
        Err(e) => return Err(Error::from(e)),
        Ok(e) => e
    };
    *state.user.lock().unwrap() = Some(user);

    Ok(next.call(req).await?)
}