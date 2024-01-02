
use serde::Serialize;

#[derive(Serialize)]
pub struct Post {
  pub _id: String,
  pub title: String,
  pub content: String,
}

#[derive(Serialize)]
pub struct CreatePostDto {
  pub title: String,
  pub content: String,
}