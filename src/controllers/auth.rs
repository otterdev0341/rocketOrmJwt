use std::{any::Any, time::SystemTime};

use rocket::{serde::{json::Json, Deserialize, Serialize}, State};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use crate::{entities::user, AppConfig};
use super::{Response, SuccessResponse, ErrorResponse};
use crate::entities::prelude::User;
use bcrypt::{hash, verify, DEFAULT_COST};
use rocket::http::Status;
use jsonwebtoken::*;
use crate::auth::AuthenticatedUser;


#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReqSignIn {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize, Responder)]
#[serde(crate = "rocket::serde")]
pub struct ResSignIn {
    token: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Claims {
    sub: i32,
    role: String,
    exp: u64,
}


#[post("/sing-in", data = "<req_sign_in>")]
pub async fn sing_in(
    db: &State<DatabaseConnection>,
    req_sign_in: Json<ReqSignIn>,
    config: &State<AppConfig>

) -> Response<Json<ResSignIn>> 
{
    let db = db as &DatabaseConnection;
    let config = config as &AppConfig;
    let u: user::Model = match User::find()
        .filter(user::Column::Email.eq(&req_sign_in.email))
        .one(db)
        .await?
    {
        Some(u) => u,
        None => {
            return Err(ErrorResponse((
                Status::Unauthorized,
                "Invalid credentials".to_string(),
            )))
        }
    };

    if !verify(&req_sign_in.password, &u.password).unwrap() {
        return Err(ErrorResponse((
            Status::Unauthorized,
            "Invalid credentials".to_string()
        )));
    }

    let claims = Claims {
        sub: u.id,
        role: "user".to_owned(),
        exp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + 4 * 60 * 60,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
    .unwrap();

    Ok(SuccessResponse((Status::Ok, Json(ResSignIn { token }))))
    
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReqSignUp {
    email: String,
    password: String,
    firstname: Option<String>,
    lastname: Option<String>,
}



#[post("/sing-up", data = "<req_sign_up>")]
pub async fn sing_up(
    db: &State<DatabaseConnection>,
    req_sign_up: Json<ReqSignUp>
) -> Response<String> {
    let db = db as &DatabaseConnection;
    
    if User::find()
        .filter(user::Column::Email.eq(&req_sign_up.email))
        .one(db)
        .await?
        .is_some()
    {
        return Err(super::ErrorResponse((
            Status::UnprocessableEntity,
            "An account exists with that email address.".to_string(),
        )));
    }
    
    User::insert(user::ActiveModel {
        email: Set(req_sign_up.email.to_owned()),
        password: Set(hash(req_sign_up.password.to_owned(), DEFAULT_COST).unwrap()),
        firstname: Set(req_sign_up.firstname.to_owned()),
        lastname: Set(req_sign_up.lastname.to_owned()),
        ..Default::default()
    })
    .exec(db)
    .await?;
    
    Ok(SuccessResponse((
        Status::Created,
        "Account created!".to_string(),
    )))

    
}


#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ResMe {
    id: i32,
    email: String,
    firstname: Option<String>,
    lastname: Option<String>,
}

#[get("/me")]
pub async fn me(db: &State<DatabaseConnection>, user: AuthenticatedUser) -> Response<Json<ResMe>> {
    let db = db as &DatabaseConnection;
    let id = user.id;
    let id = id as i32;
    let u: user::Model = User::find_by_id(id).one(db).await?.unwrap();

    Ok(SuccessResponse((
        Status::Ok,
        Json(ResMe {
            id: u.id,
            email: u.email,
            firstname: u.firstname,
            lastname: u.lastname,
        }),
    )))
}