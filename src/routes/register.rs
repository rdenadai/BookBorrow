use crate::middleware::auth::JwtValidator;
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
        )
        .service(
            web::scope("/reservations")
                .wrap(JwtValidator)
                .service(reservations::get_all)
                .service(reservations::get_one)
                .service(reservations::create)
                .service(reservations::update)
                .service(reservations::delete),
        )
        .service(
            web::scope("/users")
                .wrap(JwtValidator)
                .service(users::get_one)
                .service(users::create)
                .service(users::update)
                .service(users::delete),
        );
}
