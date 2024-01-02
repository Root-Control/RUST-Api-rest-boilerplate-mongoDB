use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Article {
    pub title: String,
    pub content: String,
}