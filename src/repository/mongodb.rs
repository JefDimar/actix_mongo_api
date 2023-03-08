use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::models::user::User;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::InsertOneResult,
    Client, Collection,
};

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(val) => val.to_string(),
            Err(_) => format!("Error getting env variable"),
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("rustDB");
        let col: Collection<User> = db.collection("Users");
        MongoRepo { col }
    }

    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };

        let user = self
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("error creating user");
        Ok(user)
    }

    pub async fn get_user(&self, id: &str) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! { "_id": obj_id };
        let user_detail = self
            .col
            .find_one(filter, None)
            .await
            .ok()
            .expect("error getting user detail");
        Ok(user_detail.unwrap())
    }
}
