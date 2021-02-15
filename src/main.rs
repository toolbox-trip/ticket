mod date;
mod handlers;
mod jwk;
mod model;
mod token;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use flexi_logger::{DeferredNow, Record};

//  env: RUST_LOG
//       JGEN_URL
//       REDIS_URL
//       DATABASE_URL
//       DATABASE_USERNAME
//       DATABASE_PASSWORD
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    flexi_logger::Logger::with_env_or_str("info")
        .format(log_format)
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

fn log_format(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    write!(
        w,
        "{:35} {} [{}] {}",
        now.now().to_rfc3339(),
        record.level(),
        record.module_path().unwrap_or("<unnamed>"),
        record.args()
    )
}
