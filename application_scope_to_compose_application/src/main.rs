use actix_web::{App, HttpServer, Scope};

mod user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let scope: Scope = user::get_scope();

    HttpServer::new(|| {
        App::new().service(&scope)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
