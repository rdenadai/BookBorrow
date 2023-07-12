mod auth;
mod constants;
mod models;
mod routes;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use routes::config::configure;
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
                    .wrap(Logger::default())
                    .wrap(Logger::new("%a %{User-Agent}i"))
                    .app_data(web::Data::new(db.clone()))
                    .configure(configure)
            })
            .bind(("0.0.0.0", 8000))?
            .run()
            .await
        }
        Err(e) => panic!("Error connecting to database: {:?}", e),
    }
}
