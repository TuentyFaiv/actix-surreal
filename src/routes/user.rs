use actix_web::{web, get, HttpResponse, Responder, Scope};

use crate::AppState;

use super::Route;

pub struct UserRouter;

impl UserRouter {
  pub fn new() -> Scope {
    web::scope("/user").configure(Self::config)
  }
}

impl Route for UserRouter {
  fn config(cfg: &mut web::ServiceConfig) {
    cfg
      .service(home)
      .service(by_id)
      .service(all);
  }
}

#[get("/")]
async fn home(data: web::Data<AppState>) -> impl Responder {
  match data.db.new_person().await {
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

#[get("/{id}/")]
async fn by_id(data: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
	let id = id.into_inner();
	log::info!("Got id: {}", id);
	let id = id.split(":").collect::<Vec<&str>>()[1];
	match data.db.get_user(id).await {
		Ok(user) => {
			HttpResponse::Ok().json(user)
		},
		Err(e) => {
			log::error!("Error getting user: {}", e);
			HttpResponse::InternalServerError().body("Error getting user")
		},
	}
}

#[get("/all")]
async fn all(data: web::Data<AppState>) -> impl Responder {
  match data.db.all_users().await {
    Ok(users) => {
      HttpResponse::Ok().json(users)
    },
    Err(e) => {
      log::error!("Error getting users: {}", e);
      HttpResponse::InternalServerError().body("Error getting users")
    },
  }
}