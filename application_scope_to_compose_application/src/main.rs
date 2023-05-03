extern crate log4rs;
use actix_web::{App, HttpServer};
use log::{info};

mod product;
mod user;

// async fn test() -> impl Responder {
//     HttpResponse::Ok().body("help!")
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    info!("Start application.");

    HttpServer::new(|| {
        App::new()
            .configure(user::get_scope)
            .configure(product::get_scope)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
