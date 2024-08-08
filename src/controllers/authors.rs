
use crate::auth::AuthenticatedUser;
use crate::entities::{author, prelude::*};
use super::{Response, SuccessResponse,ErrorResponse};
use rocket::http::Status;
use rocket::{serde::json::Json, State};
use sea_orm::*;


#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResAuthor {
    id: i32,
    firstname: String,
    lastname: String,
    bio: String,
    
}


#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResAuthorList {
    total: usize,
    authors: Vec<ResAuthor>
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReqAuthor {
    firstname: String,
    lastname: String,
    bio: String,
}

#[get("/")]
pub async fn index(
    db: &State<DatabaseConnection>,
    _user: AuthenticatedUser
) -> Response<Json<ResAuthorList>> {
    let db = db as &DatabaseConnection;

    let authors = Author::find()
        .order_by_desc(author::Column::UpdatedAt)
        .all(db)
        .await?
        .iter()
        .map(|a| ResAuthor {
            id: a.id,
            firstname: a.firstname.to_owned(),
            lastname: a.lastname.to_owned(),
            bio: a.bio.to_owned(),
        })
        .collect::<Vec<_>>();

    Ok(SuccessResponse((
        Status::Ok,
        Json(ResAuthorList {
            total: authors.len(),
            authors,
        }),
    )))
}
#[post("/", data = "<req_author>")]
pub async fn create(
    db: &State<DatabaseConnection>,
    req_author: Json<ReqAuthor>,
    user: AuthenticatedUser
) -> Response<Json<ResAuthor>> {
    let db = db as &DatabaseConnection;
    let temp_id = user.id;
    let the_id = temp_id as i32;
    let author = author::ActiveModel {
        user_id: Set(the_id),
        firstname: Set(req_author.firstname.to_owned()),
        lastname: Set(req_author.lastname.to_owned()),
        bio: Set(req_author.bio.to_owned()),
        ..Default::default()
    };

    let author = author.insert(db).await?;
    
    Ok(SuccessResponse((Status::Created, Json(ResAuthor {
        id: author.id,
        firstname: author.firstname,
        lastname: author.lastname,
        bio: author.bio,
    }))))
    
}

#[get("/<id>")]
pub async fn show(
    db: &State<DatabaseConnection>,
    id: u32,
    user: AuthenticatedUser,
) -> Response<Json<ResAuthor>> {

    let db = db as &DatabaseConnection;

    let author = Author::find_by_id((id) as i32).one(db).await?;

    let author = match author {
        Some(a) => a,
        None => {
            return Err(ErrorResponse((
                Status::NotFound,
                "No author found by this current id found".to_owned(),
            )));
        }
    };

        Ok(SuccessResponse((
            Status::Ok,
            Json(ResAuthor {
                id: author.id,
                firstname: author.firstname,
                lastname: author.lastname,
                bio: author.bio,
            })
        )))
    }

    


#[put("/<id>")]
pub async fn update(id: u32) -> Response<String> {
    todo!()
}

#[delete("/<id>")]
pub async fn delete(id: u32) -> Response<String> {
    todo!()
}
