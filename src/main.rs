use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use api::user::{create_user, get_user};
use repository::mongodb::MongoRepo;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello, world!")
}

mod api;
mod models;
mod repository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(hello)
            .service(create_user)
            .service(get_user)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
