use crate::{models::user::User, repository::mongodb::MongoRepo};
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};

#[post("/user")]
pub async fn create_user(db: Data<MongoRepo>, new_user: Json<User>) -> HttpResponse {
    let data = User {
        id: None,
        name: new_user.name.to_owned(),
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
    };
    let user_details = db.create_user(data).await;
    match user_details {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
