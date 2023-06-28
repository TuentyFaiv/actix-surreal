use serde::{Serialize, Deserialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize)]
pub struct Name<'a> {
  pub first: &'a str,
  pub last: &'a str,
}

#[derive(Debug, Serialize)]
pub struct Person<'a> {
  pub title: &'a str,
  pub name: Name<'a>,
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