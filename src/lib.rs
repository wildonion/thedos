





use async_trait::async_trait;



// https://blog.softwaremill.com/multithreading-in-rust-with-mpsc-multi-producer-single-consumer-channels-db0fc91ae3fa
// https://ryhl.io/blog/actors-with-tokio/
// https://ryhl.io/blog/async-what-is-blocking/
// todo ➔ connect to different vpn every 30 seconds to avoid hit by blocking, use a socks5 proxy list
// todo ➔ actors proc macros and traits
// ...




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

        (
            || async move{
                34
            }
        )().await;
    
        Error
    }

}