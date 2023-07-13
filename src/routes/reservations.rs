use crate::models::reservations::{
    ActiveModel as ActiveModelReservation, Entity as EntityReservation, Model as ModelReservation,
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use log::warn;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, TryIntoModel};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct DeletedRecord {
    status: bool,
    message: String,
}

#[get("")]
pub async fn get_all(db: web::Data<DatabaseConnection>) -> impl Responder {
    let connection = db.get_ref();
    match EntityReservation::find().into_json().all(connection).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => {
            warn!("Unable to load data (Reservation::get_all): {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/{id}")]
pub async fn get_one(path: web::Path<Uuid>, db: web::Data<DatabaseConnection>) -> impl Responder {
    let reservation_id = path.into_inner();
    let connection = db.get_ref();
    match EntityReservation::find_by_id(reservation_id)
        .into_json()
        .one(connection)
        .await
    {
        Ok(Some(data)) => HttpResponse::Ok().json(data),
        Ok(None) => {
            warn!("Unable to load data (Reservation::get_one): Reservation not found");
            HttpResponse::NotFound().finish()
        }
        Err(err) => {
            warn!("Unable to load data (Reservation::get_one): {}", err);
            HttpResponse::NotFound().finish()
        }
    }
}

#[post("")]
pub async fn create(
    reservation: web::Json<ModelReservation>,
    db: web::Data<DatabaseConnection>,
) -> impl Responder {
    warn!("Creating reservation: {:?}", reservation);
    let connection = db.get_ref();
    match ActiveModelReservation::from(reservation.0)
        .insert(connection)
        .await
    {
        Ok(data) => HttpResponse::Ok().json(data.try_into_model().unwrap()),
        Err(err) => {
            warn!("Unable to insert data (Reservation::create): {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[put("/{id}")]
pub async fn update(
    path: web::Path<Uuid>,
    reservation: web::Json<ModelReservation>,
    db: web::Data<DatabaseConnection>,
) -> impl Responder {
    let reservation_id = path.into_inner();
    let connection = db.get_ref();
    match EntityReservation::find_by_id(reservation_id)
        .one(connection)
        .await
    {
        Ok(Some(data)) => {
            let mut model: ActiveModelReservation = data.into();
            model.merge(reservation.0);
            match model.update(connection).await {
                Ok(data) => HttpResponse::Ok().json(data),
                Err(err) => {
                    warn!("Unable to update data (Reservation::update): {}", err);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Ok(None) => {
            warn!("Unable to load data (Reservation::update): Reservation not found");
            HttpResponse::NotFound().finish()
        }
        Err(err) => {
            warn!("Unable to load data (Reservation::update): {}", err);
            HttpResponse::NotFound().finish()
        }
    }
}

#[delete("/{id}")]
pub async fn delete(path: web::Path<Uuid>, db: web::Data<DatabaseConnection>) -> impl Responder {
    let reservation_id = path.into_inner();
    let connection = db.get_ref();
    match EntityReservation::find_by_id(reservation_id)
        .one(connection)
        .await
    {
        Ok(Some(data)) => {
            let model: ActiveModelReservation = data.into();
            match model.delete(connection).await {
                Ok(_) => HttpResponse::Ok().json(DeletedRecord {
                    status: true,
                    message: "Record deleted successfully".to_string(),
                }),
                Err(err) => {
                    warn!("Unable to delete data (Reservation::delete): {}", err);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Ok(None) => {
            warn!("Unable to load data (Reservation::delete): Reservation not found");
            HttpResponse::NotFound().finish()
        }
        Err(err) => {
            warn!("Unable to load data (Reservation::delete): {}", err);
            HttpResponse::NotFound().finish()
        }
    }
}
