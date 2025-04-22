// use warp::Filter;
use crate::use_case::api_call::GetApiCallUseCase;
use crate::domain::use_case::api_call::{GetApiCallInputType, GetApiCallUseCaseType};

pub async fn get_post_handler() -> Result<impl warp::Reply, warp::Rejection> {
    let use_case = GetApiCallUseCase;
    let input = GetApiCallInputType {}; // Create an empty input
    let output = use_case.execute(input).await; // Call the use case

    Ok(warp::reply::json(&output)) // Return the output as JSON
}