use actix_web::{web, HttpResponse, Responder};

/// 사용자 router 설정
pub fn get_scope(cfg: &mut web::ServiceConfig) {
    // cfg.service(
    //     web::resource("/").route(web::get().to(show_users))
    // );
    // cfg.service(
    //     web::resource("/john").route(web::get().to(show_detail))
    // );
    cfg.service(
        web::scope("/users")
            .route("", web::get().to(show_users))
            .route("/john", web::get().to(show_detail)),
    );
}

/// 사용자 정보를 반환
async fn show_users() -> impl Responder {
    HttpResponse::Ok().body("John, Jim, Cliff")
}

async fn show_detail() -> impl Responder {
    HttpResponse::Ok().body("John!")
}
