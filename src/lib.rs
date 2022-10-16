





use async_trait::async_trait;







pub struct Error;





#[async_trait]
pub trait Issue{

    async fn call_me(&mut self) -> Error{
        Error
    }
}

#[async_trait]
impl Issue for Error{

    async fn call_me(&mut self) -> Error{
        Error
    }

}