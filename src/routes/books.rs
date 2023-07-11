use crate::models::books::{
    ActiveModel as ActiveModelBook, Entity as EntityBook, Model as ModelBook,
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use chrono::Utc;
use log::warn;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, TryIntoModel};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct DeletedRecord {
    status: bool,
    message: String,
}

#[get("/books")]
pub async fn get_all(db: web::Data<DatabaseConnection>) -> impl Responder {
    let connection = db.get_ref();
    match EntityBook::find().into_json().all(connection).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => {
            warn!("Unable to load data (Book::get_all): {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/books/{id}")]
pub async fn get_one(path: web::Path<Uuid>, db: web::Data<DatabaseConnection>) -> impl Responder {
    let book_id = path.into_inner();
    let connection = db.get_ref();
    match EntityBook::find_by_id(book_id)
        .into_json()
        .one(connection)
        .await
    {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => {
            warn!("Unable to load data (Book::get_one): {}", err);
            HttpResponse::NotFound().finish()
        }
    }
}

#[post("/books")]
pub async fn create(
    book: web::Json<ModelBook>,
    db: web::Data<DatabaseConnection>,
) -> impl Responder {
    warn!("Creating book: {:?}", book);
    println!("Creating book: {:?}", book);
    let connection = db.get_ref();
    match ActiveModelBook::from(book.0).insert(connection).await {
        Ok(data) => HttpResponse::Ok().json(data.try_into_model().unwrap()),
        Err(err) => {
            warn!("Unable to insert data (Book::create): {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[put("/books/{id}")]
pub async fn update(
    path: web::Path<Uuid>,
    book: web::Json<ModelBook>,
    db: web::Data<DatabaseConnection>,
) -> impl Responder {
    let book_id = path.into_inner();
    let connection = db.get_ref();
    match EntityBook::find_by_id(book_id).one(connection).await {
        Ok(data) => {
            let mut model: ActiveModelBook = data.unwrap().into();
            model.title = Set(book.title.to_owned());
            model.author = Set(book.author.to_owned());
            model.year_of_publication = Set(book.year_of_publication.to_owned());
            model.available = Set(book.available.to_owned());
            model.updated_at = Set(Some(Utc::now().naive_utc()));
            match model.update(connection).await {
                Ok(data) => HttpResponse::Ok().json(data),
                Err(err) => {
                    warn!("Unable to update data (Book::update): {}", err);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Err(err) => {
            warn!("Unable to load data (Book::update): {}", err);
            HttpResponse::NotFound().finish()
        }
    }
}

#[delete("/books/{id}")]
pub async fn delete(path: web::Path<Uuid>, db: web::Data<DatabaseConnection>) -> impl Responder {
    let book_id = path.into_inner();
    let connection = db.get_ref();
    match EntityBook::find_by_id(book_id).one(connection).await {
        Ok(data) => {
            let model: ActiveModelBook = data.unwrap().into();
            match model.delete(connection).await {
                Ok(_) => HttpResponse::Ok().json(DeletedRecord {
                    status: true,
                    message: "Record deleted successfully".to_string(),
                }),
                Err(err) => {
                    warn!("Unable to update data (Book::update): {}", err);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Err(err) => {
            warn!("Unable to load data (Book::update): {}", err);
            HttpResponse::NotFound().finish()
        }
    }
}
