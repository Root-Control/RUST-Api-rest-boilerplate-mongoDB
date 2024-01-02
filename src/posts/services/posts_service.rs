use super::super::types::post::{CreatePostDto, Post};

pub fn get_hardcoded_post() -> Post {
    Post {
        _id: "hola".to_string(),
        title: "Hardcoded Post".to_string(),
        content: "Hardcoded Post content".to_string(),
    }
}

pub fn create_post(post_data: CreatePostDto) -> CreatePostDto {
    return post_data;
}