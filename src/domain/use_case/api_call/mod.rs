use serde::Serialize;

use crate::domain::entities::post::Post;


#[derive(Serialize)] 
pub struct GetApiCallInputType {}

#[derive(serde::Serialize)]
pub struct GetApiCallOutputType {
    pub articles: Vec<Post>,
}

pub trait GetApiCallUseCaseType {
    type Input;
    type Output;

    async fn execute(&self, input: Self::Input) -> Self::Output;
}