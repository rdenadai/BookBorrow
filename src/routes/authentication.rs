use crate::models::users::{Column as ColumnUser, Entity as EntityUser};
use crate::utils::default::encrypt_password;
use crate::utils::token::TokenClaims;
use actix_web::{post, web, HttpResponse, Responder};
use base64::{engine::general_purpose, Engine as _};
use jsonwebtoken::{encode, EncodingKey, Header};
use log::warn;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
pub struct JwtToken {
    token: String,
}

#[derive(Deserialize)]
pub struct Login {
    email: String,
    password: String,
}

#[post("/login")]
pub async fn login(login: web::Form<Login>, db: web::Data<DatabaseConnection>) -> impl Responder {
    let connection = db.get_ref();
    match EntityUser::find()
        .filter(
            Condition::all()
                .add(ColumnUser::Email.contains(login.email.as_str()))
                .add(
                    ColumnUser::Password
                        .contains(encrypt_password(login.password.to_string()).as_str()),
                ),
        )
        .one(connection)
        .await
    {
        Ok(Some(user)) => {
            let now = chrono::Utc::now();
            let iat = now.timestamp() as usize;
            let exp = (now
                + chrono::Duration::minutes(
                    env::var("JWT_TIMEOUT")
                        .expect("JWT_TIMEOUT: Not Found!")
                        .parse::<i64>()
                        .expect("JWT_TIMEOUT: Wrong type!"),
                ))
            .timestamp() as usize;
            let claims: TokenClaims = TokenClaims {
                sub: user.id.to_string(),
                exp,
                iat,
            };

            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(
                    env::var("JWT_SECRET")
                        .expect("JWT_SECRET: Not Found!")
                        .as_ref(),
                ),
            )
            .unwrap();
            HttpResponse::Ok().json(JwtToken {
                token: general_purpose::STANDARD_NO_PAD.encode(token),
            })
        }
        Ok(None) => {
            warn!("Unable to login (Authentication::login): User not found");
            HttpResponse::NotFound().finish()
        }
        Err(err) => {
            warn!("Unable to login (Authentication::login): {}", err);
            HttpResponse::NotFound().finish()
        }
    }
}
