



// https://blog.softwaremill.com/multithreading-in-rust-with-mpsc-multi-producer-single-consumer-channels-db0fc91ae3fa
// https://ryhl.io/blog/actors-with-tokio/
// https://ryhl.io/blog/async-what-is-blocking/





// actors uses jobq channels, multithreading and async message passing concetps
// messages must be Send Sync static and Arc<Mutex<Message>> to share between actor threads



// actors macros and traits
// ...


use crate::*;