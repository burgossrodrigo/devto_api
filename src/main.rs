use warp::Filter;
use devto_api::infra::handler::post::get_post_handler;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // Define the route
    let get_posts_route = warp::path("posts")
        .and(warp::get())
        .and_then(get_post_handler);

    // Start the warp server
    warp::serve(get_posts_route).run(([127, 0, 0, 1], 3030)).await;
}