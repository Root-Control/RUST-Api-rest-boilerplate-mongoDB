use super::super::types::article::{Article, CreateArticleDto};

pub fn get_hardcoded_article() -> Article {
    Article {
        _id: "fix you".to_string(),
        title: "Hardcoded Title".to_string(),
        content: "Hardcoded content".to_string(),
    }
}

pub fn create_article(article_data: CreateArticleDto) -> CreateArticleDto {
    return article_data;
}