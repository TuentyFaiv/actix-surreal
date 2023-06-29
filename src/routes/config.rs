use actix_web::{web, get, HttpResponse, Responder, Scope};
use serde::Deserialize;

use super::Route;

pub struct ConfigRouter;

#[derive(Debug, Deserialize)]
enum ConfigBy {
  METHOD,
  PSP,
}

#[derive(Debug, Deserialize)]
struct GetParams {
  by: Option<ConfigBy>,
}

impl ConfigRouter {
  pub fn new() -> Scope {
    web::scope("/config").configure(Self::config)
  }
}

impl Route for ConfigRouter {
  fn config(cfg: &mut web::ServiceConfig) {
    cfg
      .service(home);
  }
}

#[get("/")]
async fn home(params: web::Query<GetParams>) -> impl Responder {
  let GetParams { by } = params.into_inner();
  HttpResponse::Ok().body(format!("By {:?}", by))
}
