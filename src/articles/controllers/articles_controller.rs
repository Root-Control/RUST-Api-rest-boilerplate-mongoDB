use super::super::services::articles_service;
use super::super::types::article::CreateArticleDto;
use serde::Deserialize;
use serde_json::json;
use warp::Filter;

#[derive(Deserialize)]
struct QueryParams {
    name: String,
}

pub fn articles_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    let test_route = warp::path!("articles" / "test")
        .and(warp::get())
        .and(warp::query::<QueryParams>())
        .map(|params: QueryParams| {

            print!("Testing route");

            match params.name.as_str() {
                "hola" => {
                    let mut array = Vec::new();
                    array.push(1); // Push un número al array
                    warp::reply::json(&array)
                }
                "adios" => {
                    let mut obj = std::collections::HashMap::new();
                    obj.insert("hola", "mundo"); // Agrega una propiedad al objeto
                    warp::reply::json(&obj)
                }

                _ => warp::reply::json(&json!({"error": "Tipo de consulta no válido"})),
            }
        });

    let get = warp::path("articles")
      .and(warp::path::end())
      .and(warp::get())
      .map(|| {
        println!("Articles called");
        let article = articles_service::get_hardcoded_article();
        warp::reply::json(&article)
    });

    let post_articles = warp::path("articles")
        .and(warp::post())
        .and(warp::body::json())
        .map(|new_article: CreateArticleDto| {
            let res = articles_service::create_article(new_article);
            warp::reply::json(&res)
        });

    test_route.or(get).or(post_articles)
}
