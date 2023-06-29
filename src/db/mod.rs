use surrealdb::engine::remote::ws::{Ws, Client};
use surrealdb::opt::auth::Root;
use surrealdb::{Surreal, Result};

// static DB: Surreal<Client> = Surreal::init();

pub mod models;

use self::models::{Record, Person, Name, CreatePerson};

#[derive(Clone)]
pub struct DB {
  pub surreal: Surreal<Client>,
}

impl DB {
  pub async fn new() -> Self {
    let surreal = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();
    // let surreal = Surreal::new::<Ws>("surrealdb").await.unwrap();

    surreal.signin(Root {
      username: "root",
      password: "root"
    }).await.unwrap();

    surreal.use_ns("test").use_db("test").await.unwrap();

    // Ok()
    Self {
      surreal,
    }
  }
}