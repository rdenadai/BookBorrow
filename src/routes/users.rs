use crate::models::users::{
    ActiveModel as ActiveModelUser, Entity as EntityUser, Model as ModelUser,
};
use crate::utils::default::encrypt_password;
use crate::utils::token::decode_token;
use actix_web::HttpRequest;
use actix_web::{
    delete, error::ErrorUnauthorized, get, http::header::HeaderValue, post, put, web, Error,
    HttpResponse, Responder,
};
use log::warn;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, TryIntoModel};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct DeletedRecord {
    status: bool,
    message: String,
}

fn user_validation(token: Option<HeaderValue>, user_id: Uuid) -> Result<bool, Error> {
    match token {
        Some(token) => match decode_token(token) {
            Ok(claims) => {
                if claims.sub == user_id.to_string() {
                    Ok(true)
                } else {
                    Err(ErrorUnauthorized("Invalid user."))
                }
            }
            Err(err) => Err(err),
        },
        None => Err(ErrorUnauthorized("Missing authorization token.")),
    }
}

#[get("/{id}")]
pub async fn get_one(
    path: web::Path<Uuid>,
    req: HttpRequest,
    db: web::Data<DatabaseConnection>,
) -> impl Responder {
    let user_id = path.into_inner();
    let token = req.headers().get("Authorization").cloned();
    match user_validation(token, user_id) {
        Err(err) => {
            warn!(
                "Token user_id not match with changing user_id (User::get_one): {}",
                err
            );
            return HttpResponse::Unauthorized().finish();
        }
        _ => {}
    };

    let connection = db.get_ref();
    match EntityUser::find_by_id(user_id)
        .into_json()
        .one(connection)
        .await
    {
        Ok(Some(data)) => HttpResponse::Ok().json(data),
        Ok(None) => {
            warn!("Unable to load data (User::get_one): User not found");
            HttpResponse::NotFound().finish()
        }
        Err(err) => {
            warn!("Unable to load data (User::get_one): {}", err);
            HttpResponse::NotFound().finish()
        }
    }
}

#[post("")]
pub async fn create(
    user: web::Json<ModelUser>,
    db: web::Data<DatabaseConnection>,
) -> impl Responder {
    warn!("Creating user: {:?}", user);
    let connection = db.get_ref();
    let mut model = user.0;
    model.password = encrypt_password(model.password.to_owned());
    match ActiveModelUser::from(model).insert(connection).await {
        Ok(data) => HttpResponse::Ok().json(data.try_into_model().unwrap()),
        Err(err) => {
            warn!("Unable to insert data (user::create): {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[put("/{id}")]
pub async fn update(
    path: web::Path<Uuid>,
    req: HttpRequest,
    user: web::Json<ModelUser>,
    db: web::Data<DatabaseConnection>,
) -> impl Responder {
    let user_id = path.into_inner();
    let token = req.headers().get("Authorization").cloned();
    match user_validation(token, user_id) {
        Err(err) => {
            warn!(
                "Token user_id not match with changing user_id (User::get_one): {}",
                err
            );
            return HttpResponse::Unauthorized().finish();
        }
        _ => {}
    };

    let connection = db.get_ref();
    match EntityUser::find_by_id(user_id).one(connection).await {
        Ok(Some(data)) => {
            let mut model: ActiveModelUser = data.into();
            model.merge(user.0);
            model.password = Set(encrypt_password(model.password.unwrap()));
            match model.update(connection).await {
                Ok(data) => HttpResponse::Ok().json(data),
                Err(err) => {
                    warn!("Unable to update data (User::update): {}", err);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Ok(None) => {
            warn!("Unable to load data (User::update): User not found");
            HttpResponse::NotFound().finish()
        }
        Err(err) => {
            warn!("Unable to load data (User::update): {}", err);
            HttpResponse::NotFound().finish()
        }
    }
}

#[delete("/{id}")]
pub async fn delete(
    path: web::Path<Uuid>,
    req: HttpRequest,
    db: web::Data<DatabaseConnection>,
) -> impl Responder {
    let user_id = path.into_inner();
    let token = req.headers().get("Authorization").cloned();
    match user_validation(token, user_id) {
        Err(err) => {
            warn!(
                "Token user_id not match with changing user_id (User::get_one): {}",
                err
            );
            return HttpResponse::Unauthorized().finish();
        }
        _ => {}
    };

    let connection = db.get_ref();
    match EntityUser::find_by_id(user_id).one(connection).await {
        Ok(Some(data)) => {
            let model: ActiveModelUser = data.into();
            match model.delete(connection).await {
                Ok(_) => HttpResponse::Ok().json(DeletedRecord {
                    status: true,
                    message: "Record deleted successfully".to_string(),
                }),
                Err(err) => {
                    warn!("Unable to delete data (User::delete): {}", err);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Ok(None) => {
            warn!("Unable to load data (User::delete): User not found");
            HttpResponse::NotFound().finish()
        }
        Err(err) => {
            warn!("Unable to load data (User::delete): {}", err);
            HttpResponse::NotFound().finish()
        }
    }
}
