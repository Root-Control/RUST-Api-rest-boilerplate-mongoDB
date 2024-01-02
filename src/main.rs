mod articles;
mod posts;
use warp::Filter;

use articles::controllers::articles_controller::articles_routes;
use posts::controllers::posts_controller::posts_routes;

#[tokio::main]
async fn main() {
    let routes = articles_routes().or(posts_routes());

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}