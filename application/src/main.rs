use actix_web::{web, App, HttpServer, Responder};

async fn index() -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        // /app/index.html로 접근하는 GET method 대응
        App::new().service(web::scope("/app").route("/index.html", web::get().to(index)))
    })
    .bind(("127.0.01", 8080))?
    .run()
    .await
}
