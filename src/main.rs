mod date;
mod handlers;
mod jwk;
mod model;
mod token;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

//  env: RUST_LOG
//       JGEN_URL
//       MEMCACHED_URL
//       DATABASE_URL
//       DATABASE_USERNAME
//       DATABASE_PASSWORD
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    flexi_logger::Logger::with_env_or_str("debug")
        .start()
        .unwrap();
    log::info!("starting server on 8080...");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .data(model::ConfigContext::default())
            .configure(handlers::register)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

// fn main() {
//     let source = "123456\n\r7890";
//     let source_bytes = source.as_bytes();
//     let position = source_bytes.into_iter().position(|ch| *ch == 10).unwrap();
//     let result = &source_bytes[(position + 1)..source_bytes.len() - 2];
//     println!("{:?}", result);
// }
