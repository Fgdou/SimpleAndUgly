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
    if req.path() != "/login" {
        let state: &web::Data<AppState> = req.app_data().unwrap();

        let cookie = match req.cookie("token") {
            None => return Err(Error::from(AuthError::TokenNotExist)),
            Some(cookie) => cookie
        };
        let user = state.auth.authenticate(cookie.value())?;
        *state.user.lock().unwrap() = Some(user);
    }
    let next = Rc::new(next);
    Ok(next.call(req).await?)
}