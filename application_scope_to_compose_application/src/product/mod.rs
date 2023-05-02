use actix_web::{web, HttpResponse, Responder};

pub fn get_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .route("", web::get().to(show_products))
            .route("/banana", web::get().to(show_detail)),
    );
}

async fn show_products() -> impl Responder {
    HttpResponse::Ok().body("Apple, Melon, Banana")
}

async fn show_detail() -> impl Responder {
    HttpResponse::Ok().body("Banana")
}
