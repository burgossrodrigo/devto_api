use crate::domain::entities::post::Post;


pub struct GetPostInput {}

pub struct GetPostOutput {
    pub posts: Vec<Post>,
}

pub trait UseCase {
    type Input;
    type Output;

    fn execute(&self, input: Self::Input) -> Self::Output;
}