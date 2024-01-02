
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct Article {
  pub _id: String,
  pub title: String,
  pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateArticleDto {
  pub title: String,
  pub content: String,
}