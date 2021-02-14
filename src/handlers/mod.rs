mod ticket_handler;

use actix_web::web::ServiceConfig;

pub fn register(cfg: &mut ServiceConfig) {
    // cfg.service(current_ticket).service(jwks);
    ticket_handler::register(cfg);
}
