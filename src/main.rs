use warp::Filter;
use devto_api::domain::use_case::{get_posts::{GetPostInput, GetPostUseCase}, use_case::UseCase};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // Define the route
    let get_posts_route = warp::path("posts")
        .and(warp::get())
        .and_then(|| async {
            let use_case = GetPostUseCase;
            let input = GetPostInput {}; // Create an empty input
            let output = use_case.execute(input).await; // Await the async execution

            Ok::<_, warp::Rejection>(warp::reply::json(&output)) // Return the output as JSON
        });

    // Start the warp server
    warp::serve(get_posts_route).run(([127, 0, 0, 1], 3030)).await;
}