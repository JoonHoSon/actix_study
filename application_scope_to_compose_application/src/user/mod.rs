use actix_web::{get, HttpResponse, Scope, web};

#[get("/")]
async fn show_users() -> HttpResponse {
    HttpResponse::Ok().body("John, Jim, Cliff")
}


pub fn get_scope() -> Scope {
    web::scope("/users").service(show_users)
}