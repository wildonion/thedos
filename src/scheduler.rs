




/*


➔ actors uses jobq channels to send message events asyncly between other actors and the system to execute them inside their free thread from the thread pool
➔ messages must be Send Sync static and Arc<Mutex<Message>> to share between actor threads
➔ mpsc means multiple threads can read the data which i Send + Sync + 'static or multiple sender can be cloned but only one thread or receiver can mutate the data
➔ The three causes of data races
    • Two or more pointers access the same data at the same time.
    • At least one of the pointers is being used to write to the data.
    • There’s no mechanism being used to synchronize access to the data
➔ The three rules of ownership
    • Each value in Rust has a variable that’s called its owner.
    • There can only be one owner at a time.
    • When the owner goes out of scope, the value will be dropped.
➔ The two rules of references
    • At any given time, you can have either one mutable reference or any number of immutable references.
    • References must always be valid.


*/









pub mod _async{


    // async worker pool scheduler using tokio based on mpsc jobq channel

    use crate::*;
    
    


    // https://blog.softwaremill.com/multithreading-in-rust-with-mpsc-multi-producer-single-consumer-channels-db0fc91ae3fa
    // https://ryhl.io/blog/actors-with-tokio/
    // https://ryhl.io/blog/async-what-is-blocking/
    



    pub struct Actor<E>{
        count: usize, // number of workers
        sender: mpsc::UnboundedSender<Result<(), E>>, // sender async side with no byte limitation
        receiver: mpsc::UnboundedReceiver<Result<(), E>>, // receiver async side with no byte limitation
    }


    impl<E: Send + 'static> Actor<E>{ // E can be shared between threads

        pub fn new() -> Self{
            let (sender, 
                receiver) = mpsc::unbounded_channel(); // async mpsc jobq channel channel with no byte limitation to avoid deadlocks and race conditions
            Actor{
                count: 0, // will be equaled to the number of workers by solving all the jobs which are comming to the downside of the mpsc jobq channel
                sender,
                receiver
            }
        }
        

        pub fn schedule<T>(){

            todo!() // ➔ schedule attack every 40 seconds after any error

        }


        pub fn broadcast(){
            
            todo!() // ➔ use tokio::sync::broadcast
        
        }


        pub fn spawn<T>(&mut self, task: T)
            where 
                T: Future<Output=Result<(), E>> + Send + 'static, // T can be shared between threads
                T::Output: Is<Type = Result<(), E>>, // T is a future and now we can access the Output type to make sure that is of type Result<(), E> - T::Output is the GAT of the Future trait
                {
                    let sender = self.sender.clone();
                    tokio::spawn(async move{ // spawn the task inside tokio green threads
                        let res = task.await;
                        match sender.send(res.into_val()){
                            Ok(()) => (),
                            Err(_) => panic!("Impossible Panic for Sender"),
                        }
                    });
                    self.count += 1;
                }


        pub async fn execute(mut self) -> Result<(), E>{

            std::mem::drop(self.sender); // make sure that the sender is dead since we want to receive all the messages and avoid deadlocks and race condition
            let mut index = 0;

            loop{ // we can use while let Some() syntax
                match self.receiver.recv().await{
                    Some(Ok(())) => {
                        assert!(index < self.count);
                    }
                    Some(Err(e)) => {
                        assert!(index < self.count);
                        return Err(e);
                    }
                    None => {
                        assert_eq!(index, self.count);
                        break Ok(()); // return this to the main
                    }
                }
                index+=1;
            }

        }

    }
}










pub mod sync{


    // a sync task scheduler (worker pool) with mpsc as the jobq channel protocol


    use crate::*;
    




    type Job = Box<dyn FnOnce() + Send + 'static>; //-- a job is of type closure which must be Send and static across all threads inside a Box on the heap

    struct Worker{
        id: Uuid,
        thread: Option<thread::JoinHandle<()>>, //// thread is of type JoinHandld struct which return nothing or ()
    }

    pub struct ThreadPool {
        workers: Vec<Worker>,
        sender: std_mpsc::Sender<Message>, //-- all sends will be asynchronous and they never block
    }

    enum Message {
        NewJob(Job),
        Terminate,
    }

    impl ThreadPool{
        
        pub fn new(size: usize) -> ThreadPool {
            assert!(size > 0);
            let (sender, receiver) = std_mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver)); //-- reading and writing from an IO must be mutable thus the receiver must be inside a Mutex cause data inside Arc can't be borrows as mutable since the receiver read operation is a mutable process
            let mut workers = Vec::with_capacity(size); //-- capacity is not always equals to the length and the capacity of this vector is same as the maximum size based on the system arch, on 32 bits arch usize is 4 bytes and on 64 bits arch usize is 8 bytes
            for _ in 0..size { //-- since the receiver is not bounded to trait Clone we must clone it using Arc in each iteration cause we want to share it between multiple threads to get what the sender has sent 
                workers.push(Worker::new(Uuid::new_v4(), Arc::clone(&receiver)));
            }
            ThreadPool{workers, sender}
        }

        pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static { //-- calling this method means send the incoming task from the process through the mpsc sender to down side of the channel in order to block a free thread using the receiver on locking the mutex
            let job = Box::new(f);
            self.sender.send(Message::NewJob(job)).unwrap(); //-- by executing the task handler sender will send a job asynchronously and only one receiver at a time can get that job and solve it by locking on the mutex to block the choosen thread since thread safe programming is all about this pattern!
        }
    }

    impl Drop for ThreadPool{ //-- hitting CTRL + C can drop the lifetime also
        fn drop(&mut self) { //-- destructor for ThreadPool struct 
            info!("Sending terminate message to all workers.");
            for _ in &self.workers {
                self.sender.send(Message::Terminate).unwrap();
            }
            info!("Shutting down all workers.");
            for worker in &mut self.workers {
                info!("Shutting down worker {}", worker.id);
                if let Some(thread) = worker.thread.take(){ //-- take() takes the value out of the option, leaving a None in its place
                    thread.join().unwrap(); //-- joining on thread will block the current thread to get the computation result and stop the thread from being processed in the background
                }
            }
        }
    }

    impl Worker{
        fn new(id: Uuid, receiver: Arc<Mutex<std_mpsc::Receiver<Message>>>) -> Worker {
            let thread = thread::spawn(move || loop { //-- spawning a thread inside the new() method and waiting for the receiver until a job becomes available to down side of the channel
                while let Ok(message) = receiver.lock().unwrap().recv(){ //-- iterate through the receiver to get all incoming messages - since other thread shouldn't mutate this message while this thread is waiting for the job we must do a locking on the message received from the sender to acquire the mutex by blocking the current thread to avoid being in dead lock, shared state and race condition situation
                    match message {
                        Message::NewJob(job) => {
                            info!("Worker {} got a job; executing.", id);
                            job(); //-- this might be an async task or job spawned by the tokio spawner in the background
                        }
                        Message::Terminate => {
                            info!("Worker {} was told to terminate.", id);
                            break;
                        }
                    }
                }
            });
            Worker {
                id,
                thread: Some(thread),
            }
        }
    }


}