use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::models::user::User;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
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

    pub async fn update_user(&self, id: &str, new_user: User) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set": {
                "id": new_user.id,
                "name": new_user.name,
                "location": new_user.location,
                "title": new_user.title,
            },
        };

        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("error updating user");
        Ok(updated_doc)
    }

    pub async fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .delete_one(filter, None)
            .await
            .ok()
            .expect("error deleting user");
        Ok(user_detail)
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let mut cursors = self
            .col
            .find(None, None)
            .await
            .ok()
            .expect("erorr getting list of users");
        let mut users: Vec<User> = Vec::new();
        while let Some(user) = cursors
            .try_next()
            .await
            .ok()
            .expect("error mapping through cursors")
        {
            users.push(user)
        }

        Ok(users)
    }
}
