use crate::constants::TokenClaims;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    Error,
};
use futures::future::LocalBoxFuture;
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::env;
use std::future::{ready, Ready};

pub struct JwtValidator;

impl<S, B> Transform<S, ServiceRequest> for JwtValidator
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtValidatorMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtValidatorMiddleware { service }))
    }
}

pub struct JwtValidatorMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for JwtValidatorMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token = req.headers().get("Authorization").cloned();
        let fut = self.service.call(req);
        Box::pin(async move {
            match token {
                Some(token) => {
                    let res = fut.await?;
                    let token_str = token.to_str().unwrap().replace("Bearer ", "");
                    match decode::<TokenClaims>(
                        &token_str,
                        &DecodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_ref()),
                        &Validation::default(),
                    ) {
                        Ok(_) => Ok(res),
                        Err(_) => Err(ErrorUnauthorized("Invalid or missing authorization token.")),
                    }
                }
                None => Err(ErrorUnauthorized("Missing authorization token.")),
            }
        })
    }
}
