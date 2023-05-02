use actix_web::{get, HttpResponse, Responder, web};

pub async fn get_scope() -> impl Responder {
    HttpResponse::Ok().body("John, Jim, Cliff")
}


pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/user/show_users").route()
    )
}