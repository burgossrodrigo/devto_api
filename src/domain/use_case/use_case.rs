pub trait UseCase {
    type Input;
    type Output;

    async fn execute(&self, input: Self::Input) -> Self::Output;
}
