use std::borrow::Cow;

use serde::{Serialize, Deserialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct Name {
  pub first: Cow<'static, str>,
  pub last: Cow<'static, str>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
  pub id: Thing,
  pub title: Cow<'static, str>,
  pub name: Name,
  pub marketing: bool,
}

#[derive(Debug, Serialize)]
pub struct CreatePerson<'a> {
  pub title: &'a str,
  pub name: Name,
  pub marketing: bool,
}

#[derive(Debug, Serialize)]
pub struct Responsibility {
  marketing: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
  #[allow(dead_code)]
  id: Thing,
}