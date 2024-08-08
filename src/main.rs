
use fairings::cors::{options, CORS};
use migrator::Migrator;

use rocket::http::Status;
use sea_orm_migration::prelude::*;
use dotenv::dotenv;
use controllers::{Response, SuccessResponse};
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;
mod migrator;
mod db;
mod entities;
mod controllers;
mod fairings;
mod auth;

pub struct AppConfig {
    db_host: String,
    db_port: String,
    db_username: String,
    db_password: String,
    db_database: String,
    jwt_secret: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            db_host: std::env::var("BOOKSTORE_DB_HOST").unwrap_or("192.168.1.51".to_string()),
            db_port: std::env::var("BOOKSTORE_DB_PORT").unwrap_or("3306".to_string()),
            db_username: std::env::var("BOOKSTORE_DB_USERNAME").unwrap_or("vmotter".to_string()),
            db_password: std::env::var("BOOKSTORE_DB_PASSWORD").unwrap_or("vmotter".to_string()),
            db_database: std::env::var("BOOKSTORE_DB_DATABASE").unwrap_or("BOOKSTORE".to_string()),
            jwt_secret: std::env::var("BOOKSTORE_JWT_SECRET").expect("Please set the BOOKSTORE_JWT_SECRET"),
        }
    }
}


#[get("/")]
fn index() -> Response<String> {
    Ok(SuccessResponse((Status::Ok, "Hello world".to_string())))
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
        .attach(CORS)
        .manage(db)
        .manage(config)
        .mount("/", routes![options])
        .mount("/", routes![index])
        .mount("/auth", routes![
            controllers::auth::sing_in,
            controllers::auth::sing_up,
            controllers::auth::me,
        ])
        .mount("/authors", routes![
            controllers::authors::index,
            controllers::authors::create,
            controllers::authors::show,
            controllers::authors::update,
            controllers::authors::delete,
        ])
        .mount("/books", routes![
            controllers::books::index,
            controllers::books::create,
            controllers::books::show,
            controllers::books::update,
            controllers::books::delete,
        ])
}