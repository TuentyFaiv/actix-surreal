use surrealdb::engine::remote::ws::{Ws, Client};
use surrealdb::opt::auth::Root;
use surrealdb::{Surreal, Result};

// static DB: Surreal<Client> = Surreal::init();

pub mod models;

use self::models::{Record, Person, Name, CreatePerson};

pub struct DB {
  surreal: Surreal<Client>,
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
  pub async fn new_person(&self) -> Result<Record> {
    let person = CreatePerson {
      title: "Mr.",
      name: Name {
        first: "John".into(),
        last: "Doe".into(),
      },
      marketing: true,
    };

    let created: Record = self.surreal.create("person")
      .content(&person)
      .await?;

    Ok(created)
  }
  pub async fn get_user(&self, id: &str) -> Result<Record> {
    let user = self.surreal.select(("person", id)).await?;

    Ok(user)
  }

  pub async fn all_users(&self) -> Result<Vec<Person>> {
    // Perform a custom advanced query
    let users = self.surreal
      .select("person")
      .await?;

    Ok(users)
  }
}