use crate::auth::JwtValidator;
use crate::routes::*;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(index::index)
        .service(authentication::login)
        .service(
            web::scope("/books")
                .wrap(JwtValidator)
                .service(books::get_all)
                .service(books::get_one)
                .service(books::create)
                .service(books::update)
                .service(books::delete),
        );
}
