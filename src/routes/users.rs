use actix_web::{get, HttpResponse, Responder, web};
use njord::sqlite::{self};

#[get("/users/all")]
pub async fn all() -> impl Responder {
    HttpResponse::Ok().body("Getting all users!")
}

#[get("/users/{id}")]
pub async fn get(path: web::Path<(u32,)>) -> impl Responder {
    let post_id = path.into_inner().0;

    let db_name = "database.db";
    let db_path = Path::new(&db_name);
    let columns = vec!["id".to_string(), "username".to_string(), "email".to_string(), "address".to_string()];
    
    match sqlite::open(db_path) {
        Ok(c) => {            
            let result: Result<Vec<User>> = sqlite::select(c, columns).from(User::default()).build();
            
            match result {
                Ok(result) => HttpResponse::Ok().body(format!("{}", result)),
                Err(error) => panic!("Failed to get users: {:?}", error),
            };
        }
        Err(err) => eprintln!("Error opening the database: {}", err),
    }
    
    HttpResponse::Ok().body(format!("Getting users with ID: {}", post_id))
}

