// use std::sync::Arc;

use actix_web::{get, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod db;
mod services;
mod routes;

use db::{DB};
use routes::config::ConfigRouter;
use routes::transaction::TransactionRouter;
use routes::user::UserRouter;

// #[derive(Clone)]
// struct AppState {
// 	db: Arc<DB>,
// }

#[get("/")]
async fn hello() -> impl Responder {
	HttpResponse::Ok().body("Hello world!")
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
	log::debug!("Debugging enabled");

	// // State
	// let state = AppState {
	// 	db: Arc::new(DB::new().await),
	// };
	// DB
	let db = DB::new().await;

	// let state = state.clone();

	HttpServer::new(move || {
		App::new()
			.service(hello)
			.service(ConfigRouter::new())
			.service(TransactionRouter::new())
			.service(UserRouter::new(db.clone()))
			.wrap(Logger::default())
			// .app_data(web::Data::new(state.clone()))
	})
		.bind_openssl(("127.0.0.1", 4000), builder)?
		// .bind(("127.0.0.1", 4000))?
		.run()
		.await
}
