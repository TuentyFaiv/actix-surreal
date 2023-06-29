use actix_web::{web, get, HttpResponse, Responder, Scope};
// use serde::Deserialize;

use super::Route;

pub struct TransactionRouter;

impl TransactionRouter {
  pub fn new() -> Scope {
    web::scope("/transaction").configure(Self::config)
  }
}

impl Route for TransactionRouter {
  fn config(cfg: &mut web::ServiceConfig) {
    cfg
      .service(home);
  }
}

#[get("/")]
async fn home() -> impl Responder {
  HttpResponse::Ok().body("Hello world from transaction!")
}