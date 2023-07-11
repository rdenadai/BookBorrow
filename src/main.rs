mod models;
mod routes;

use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use routes::books::{
    create as book_create, delete as book_delete, get_all as book_get_all, get_one as book_get_one,
    update as book_update,
};
use routes::index::index;
use sea_orm::Database;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url: String = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set")
        .parse()
        .unwrap();
    println!("{}", database_url);
    match Database::connect(database_url).await {
        Ok(db) => {
            // Database migrations
            println!("Database connected");
            Migrator::up(&db, None).await.unwrap();

            println!("Starting server");
            // Run http server
            HttpServer::new(move || {
                App::new()
                    .app_data(web::Data::new(db.clone()))
                    .service(index)
                    .service(book_get_all)
                    .service(book_get_one)
                    .service(book_create)
                    .service(book_update)
                    .service(book_delete)
            })
            .bind(("0.0.0.0", 8000))?
            .run()
            .await
        }
        Err(e) => panic!("Error connecting to database: {:?}", e),
    }
}
