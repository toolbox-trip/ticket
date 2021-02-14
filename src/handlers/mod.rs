mod ticket_handler;

use actix_web::web::ServiceConfig;

pub fn register(cfg: &mut ServiceConfig) {
    ticket_handler::register(cfg);
}
