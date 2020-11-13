use actix_web::web::{self, ServiceConfig};
use actix_web::{get, HttpResponse, Responder};

use crate::error::Result;
use crate::model;
use crate::utilities;

pub fn register(cfg: &mut ServiceConfig) {
    cfg.service(current_ticket).service(jwks);
}

#[get("/tickets/current")]
async fn current_ticket(
    state: web::Data<model::ConfigContext>,
    request: web::HttpRequest,
) -> Result<String> {
    let token = utilities::token::generate_token(&state).await?;
    Ok(format!("{}\n{}", request.path(), token))
}

#[get("/jwks")]
async fn jwks(state: web::Data<model::ConfigContext>) -> impl Responder {
    // TODO: get all jwk
    // TODO: generate json array
    HttpResponse::Ok().body("/jwks")
}
