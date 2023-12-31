use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

pub struct SigningSession {
    pub id: String,
    pub key: String,
    pub message: [u8; 32]
}

#[get("/round1")]
async fn round1() -> impl Responder {
    HttpResponse::Ok().body("Round 1")
}

#[get("/round2")]
async fn round2() -> impl Responder {
    HttpResponse::Ok().body("Round 2")
}

#[get("/round3")]
async fn round3() -> impl Responder {
    HttpResponse::Ok().body("Round 3")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/sign")
            .service(round1)
            .service(round2)
            .service(round3)
    );
}