use crate::domain::entities::post::Post;
use crate::domain::use_case::use_case::UseCase;
use serde::Serialize;

pub struct GetPostInput {}

#[derive(Serialize)]
pub struct GetPostOutput {
    pub posts: Vec<Post>,
}

pub struct GetPostUseCase;

impl UseCase for GetPostUseCase {
    type Input = GetPostInput;
    type Output = GetPostOutput;

    async fn execute(&self, _input: Self::Input) -> Self::Output {
        // Here you would typically call a repository or service to get the posts
        // For now, we'll just return an empty vector
        GetPostOutput { posts: vec![] }
    }
}