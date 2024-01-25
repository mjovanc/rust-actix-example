use actix_web::{get, HttpResponse, Responder, web};

#[get("/categories/all")]
pub async fn all() -> impl Responder {
    HttpResponse::Ok().body("Getting all categories!")
}

#[get("/categories/{id}")]
pub async fn get(path: web::Path<(u32,)>) -> impl Responder {
    let post_id = path.into_inner().0;
    HttpResponse::Ok().body(format!("Getting category with ID: {}", post_id))
}

