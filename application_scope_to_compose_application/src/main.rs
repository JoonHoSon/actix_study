use actix_web::{App, HttpServer};

mod product;
mod user;

// async fn test() -> impl Responder {
//     HttpResponse::Ok().body("help!")
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // HttpServer::new(|| App::new().service(web::scope("/users").route("", web::get().to(test))))
    //     .bind(("127.0.0.1", 8080))?
    //     .run()
    //     .await
    HttpServer::new(|| {
        App::new()
            .configure(user::get_scope)
            .configure(product::get_scope)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
