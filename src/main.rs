use actix_web::middleware::Logger;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/tickets/current")]
async fn current_ticket() -> impl Responder {
    // TODO: get today private jwk
    // TODO: generate jwt
    HttpResponse::Ok().body("Hello world!")
}

#[get("/jwks")]
async fn jwks() -> impl Responder {
    // TODO: get all jwk
    // TODO: generate json array
    HttpResponse::Ok().body("/jwks")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("start");
    flexi_logger::Logger::with_env_or_str("debug")
        .start()
        .unwrap();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(current_ticket)
            .service(jwks)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
