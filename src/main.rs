mod constants;
mod middleware;
mod models;
mod routes;
mod utils;

use actix_web::middleware::Logger;
use actix_web::{middleware::Compress, web::Data, App, HttpServer};

use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use routes::register::configure;
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

            // Run http server
            println!("Starting server");
            HttpServer::new(move || {
                App::new()
                    .wrap(Compress::default())
                    .wrap(Logger::default())
                    .app_data(Data::new(db.clone()))
                    .configure(configure)
            })
            .bind(("0.0.0.0", 8000))?
            .run()
            .await
        }
        Err(e) => panic!("Error connecting to database: {:?}", e),
    }
}
