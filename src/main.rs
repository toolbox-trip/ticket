use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

mod ticket;

struct State {
    jgen_url: String,
}

#[get("/tickets/current")]
async fn current_ticket(state: web::Data<State>, request: web::HttpRequest) -> impl Responder {
    // TODO: get today private jwk
    // TODO: generate jwt
    HttpResponse::Ok().body(format!("{}\n{}", request.path(), &state.jgen_url))
}

#[get("/jwks")]
async fn jwks() -> impl Responder {
    // TODO: get all jwk
    // TODO: generate json array
    HttpResponse::Ok().body("/jwks")
}

/// env: RUST_LOG
///      JGEN_URL
///      MEMCACHED_URL
///      DATABASE_URL
///      DATABASE_USERNAME
///      DATABASE_PASSWORD
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("starting server on 8080");
    flexi_logger::Logger::with_env_or_str("debug")
        .start()
        .unwrap();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .data(State {
                // jgen_url: Option<String> = std::env::var("JGEN_URL").ok();
                // memcached_url: Option<String> = std::env::var("MEMCACHED_URL").ok();
                jgen_url: "http://localhost:5000".to_string(),
            })
            .service(current_ticket)
            .service(jwks)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
