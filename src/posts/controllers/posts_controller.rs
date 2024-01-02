use warp::Filter;
use super::super::services::posts_service;

pub fn posts_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("posts")
        .and(warp::get())
        .map(|| {
            let posts = posts_service::get_hardcoded_post();
            warp::reply::json(&posts)
        })
}
