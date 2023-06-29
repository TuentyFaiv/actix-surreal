use surrealdb::{Result, Surreal};
use surrealdb::engine::remote::ws::Client;

use crate::db::models::{CreatePerson, Record, Person};

pub struct UserService {
  surreal: Surreal<Client>
}

impl UserService {
  pub fn new(surreal: Surreal<Client>) -> Self {
    Self {
      surreal
    }
  }
  pub async fn new_person(&self, person: CreatePerson) -> Result<Record> {
    let created = self.surreal.create("person")
      .content(&person)
      .await?;

    Ok(created)
  }
  pub async fn get_user(&self, id: &str) -> Result<Person> {
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