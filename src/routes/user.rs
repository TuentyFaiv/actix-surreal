// use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder, Scope};

use crate::{services::users::UserService, db::{DB, models::CreatePerson}};

// use super::Route;

pub struct UserRouter;

impl UserRouter {
  pub fn new(db: DB) -> Scope {
    let service = UserService::new(db.surreal);

    web::scope("/user").configure(|config| {
      config
        .app_data(web::Data::new(service))
        .service(
          web::resource("/")
            .route(web::post().to(Self::create))
        )
        .service(
          web::resource("/all")
            .route(web::get().to(Self::all))
        )
        .service(
          web::resource("/{id}")
            .route(web::get().to(Self::by_id))
        )
        ;
    })
  }

  async fn create(service: web::Data<UserService>, person: web::Json<CreatePerson>) -> impl Responder {
    match service.new_person(person.into_inner()).await {
      Ok(person) => {
        log::info!("Created person");
        HttpResponse::Ok().json(person)
      },
      Err(e) => {
        log::error!("Error creating person: {}", e);
        HttpResponse::InternalServerError().body("Error creating person")
      },
    }
  }
  async fn by_id(service: web::Data<UserService>, id: web::Path<String>) -> impl Responder {
    let id = id.into_inner();
    log::info!("Got id: {}", id);
    let id = id.split(":").collect::<Vec<&str>>()[1];
    match service.get_user(id).await {
      Ok(user) => {
        HttpResponse::Ok().json(user)
      },
      Err(e) => {
        log::error!("Error getting user: {}", e);
        HttpResponse::InternalServerError().body("Error getting user")
      },
    }
  }
  async fn all(service: web::Data<UserService>) -> impl Responder {
    match service.all_users().await {
      Ok(users) => {
        HttpResponse::Ok().json(users)
      },
      Err(e) => {
        log::error!("Error getting users: {}", e);
        HttpResponse::InternalServerError().body("Error getting users")
      },
    }
  }
}
