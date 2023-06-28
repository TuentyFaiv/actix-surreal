use std::sync::Arc;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod db;

use db::{DB};

#[derive(Clone)]
struct AppState {
	db: Arc<DB>,
}

#[get("/")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
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

#[get("/user/{id}/")]
async fn get_user(data: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
	// println!("{:?}", id);
	let id = id.into_inner();
	log::info!("Got id: {}", id);
	let id = id.split(":").collect::<Vec<&str>>()[1];
	match data.db.get_user(id).await {
		Ok(user) => {
			log::info!("Got user");
			HttpResponse::Ok().json(user)
		},
		Err(e) => {
			log::error!("Error getting user: {}", e);
			HttpResponse::InternalServerError().body("Error getting user")
		},
	}
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	// TLS setup
	let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
	builder
		.set_private_key_file("nopass.pem", SslFiletype::PEM)
		.unwrap();
	builder.set_certificate_chain_file("cert.pem").unwrap();

	// Logger
	env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

	log::info!("Starting server at https://127.0.0.1:4000");

	// State
	let state = AppState {
		db: Arc::new(DB::new().await),
	};

	HttpServer::new(move || {
		App::new()
			.service(hello)
			.service(get_user)
			.wrap(Logger::default())
			.app_data(web::Data::new(state.clone()))
	})
		.bind_openssl(("127.0.0.1", 4000), builder)?
		.run()
		.await
}
