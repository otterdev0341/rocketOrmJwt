
use migrator::Migrator;

use sea_orm_migration::prelude::*;
use dotenv::dotenv;
#[macro_use] extern crate rocket;

mod migrator;
mod db;
mod entities;

pub struct AppConfig {
    db_host: String,
    db_port: String,
    db_username: String,
    db_password: String,
    db_database: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            db_host: std::env::var("BOOKSTORE_DB_HOST").unwrap_or("192.168.1.51".to_string()),
            db_port: std::env::var("BOOKSTORE_DB_PORT").unwrap_or("3306".to_string()),
            db_username: std::env::var("BOOKSTORE_DB_USERNAME").unwrap_or("vmotter".to_string()),
            db_password: std::env::var("BOOKSTORE_DB_PASSWORD").unwrap_or("vmotter".to_string()),
            db_database: std::env::var("BOOKSTORE_DB_DATABASE").unwrap_or("BOOKSTORE".to_string()),
        }
    }
}


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let config = AppConfig::default();

    let db = match db::connect(&config).await {
        Ok(db) => db,
        Err(err) => panic!("error : {}",err)
    };
    match Migrator::up(&db, None).await {
        Err(err) => panic!("Error{}",err),
        Ok(_) => ()
    }

    rocket::build()
        .mount("/", routes![index])
}