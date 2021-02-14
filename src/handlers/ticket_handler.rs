use crate::model;
use crate::token;
use actix_web::web::{self, ServiceConfig};
use actix_web::{get, HttpResponse, Responder};

pub fn register(cfg: &mut ServiceConfig) {
    cfg.service(current_ticket).service(jwks);
}

#[get("v1/tickets/current")]
async fn current_ticket(
    state: web::Data<model::ConfigContext>,
    request: web::HttpRequest,
) -> impl Responder {
    let token_result = token::generate_token(&state).await;
    match token_result {
        Ok(token) => HttpResponse::Ok().body(token),
        Err(err) => {
            log::error!("{:?}", err);
            HttpResponse::InternalServerError().body("fail to get ticket")
        }
    }
}

#[get("v1/jwks")]
async fn jwks(state: web::Data<model::ConfigContext>) -> impl Responder {
    let result = token::all_jwk(&state).await;
    match result {
        Ok(jwks) => HttpResponse::Ok().body(jwks),
        Err(err) => {
            log::error!("{:?}", err);
            HttpResponse::InternalServerError().body("fail to get all jwks")
        }
    }
}
