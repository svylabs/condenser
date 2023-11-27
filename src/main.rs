use actix_web::{App, HttpServer, web};
use condense::test;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(test::init_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}