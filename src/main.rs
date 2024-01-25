mod schema;
mod routes;
mod models;

use actix_web::{App, HttpServer, Responder};
use crate::routes::{posts, profiles, tags, users};
use crate::routes::categories;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(posts::all)
            .service(posts::get)
            .service(categories::all)
            .service(categories::get)
            .service(tags::all)
            .service(tags::get)
            .service(users::all)
            .service(users::get)
            .service(profiles::all)
            .service(profiles::get)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}