use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

struct AppStateWithCounter {
    counter: Mutex<i32>,
}

async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut count = data.counter.lock().unwrap();
    *count += 1;

    format!("Request number : {count}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 전역(global) 공유 용도일 경우 HttpServer::new 이전(외부)에서 생성하여
    // clone으로 전달해야 함
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
