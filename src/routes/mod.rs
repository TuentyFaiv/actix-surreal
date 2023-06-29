use actix_web::{web};

pub mod config;
pub mod transaction;
pub mod user;

trait Route {
  fn config(cfg: &mut web::ServiceConfig);
}