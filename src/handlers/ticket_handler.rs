use crate::model;
use crate::token;
use actix_web::web::{self, ServiceConfig};
use actix_web::{get, HttpResponse, Responder};
use web::{Data, HttpRequest};

pub fn register(cfg: &mut ServiceConfig) {
    cfg.service(current_ticket).service(jwks).service(next);
}

#[get("v1/tickets/current")]
async fn current_ticket(
    g_ctx: Data<model::GlobalContext>,
    request: HttpRequest,
) -> impl Responder {
    let token_result = token::generate_token(&g_ctx).await;
    match token_result {
        Ok(token) => HttpResponse::Ok().body(token),
        Err(err) => {
            log::error!("{:?}", err);
            HttpResponse::InternalServerError().body("fail to get ticket")
        }
    }
}

#[get("v1/jwks")]
async fn jwks(g_ctx: Data<model::GlobalContext>) -> impl Responder {
    let result = token::all_jwk(&g_ctx).await;
    match result {
        Ok(jwks) => HttpResponse::Ok().body(jwks),
        Err(err) => {
            log::error!("{:?}", err);
            HttpResponse::InternalServerError().body("fail to get all jwks")
        }
    }
}

#[get("v1/tickets/next")]
async fn next(g_ctx: Data<model::GlobalContext>, request: HttpRequest) -> impl Responder {
    let token = request.headers().get("authorization");
    if token.is_none() {
        return HttpResponse::Unauthorized().body("unauthorized");
    }
    let t = token.unwrap().to_str().unwrap();
    let v_result = token::validate_jwt(&g_ctx, t).await;

    match v_result {
        Ok(ok) => {
            log::debug!("{:?}", ok);
            if ok {
                HttpResponse::Ok().body("pass validation")
            } else {
                HttpResponse::BadRequest().body("fail validation")
            }
        }
        Err(err) => {
            log::error!("{:?}", err);
            HttpResponse::InternalServerError().body("cannot run validation")
        }
    }
}
