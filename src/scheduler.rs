



use crate::*; // loading all defined crates, structs and functions from the root crate which is lib.rs in our case








// async worker pool scheduler using tokio based on mpsc jobq channel




pub struct Workers<E>{
    count: usize, // number of workers
    sender: mpsc::UnboundedSender<Result<(), E>>, // sender async side with no byte limitation
    receiver: mpsc::UnboundedReceiver<Result<(), E>>, // receiver async side with no byte limitation
}


impl<E: Send + 'static> Workers<E>{ // E can be shared between threads

    pub fn new(size: usize) -> Self{
        let (sender, receiver) = mpsc::unbounded_channel(); // async mpsc jobq channel channel with no byte limitation to avoid deadlocks and race conditions
        Workers{
            count: 0,
            sender,
            receiver
        }
    }
    

    pub fn spawn<T>(&mut self, task: T)
        where 
            T: Future<Output=Result<(), E>> + Send + 'static, // T can be shared between threads
            T::Output: Is<Type = Result<(), E>>, // T is a future and now we can access the Output type to make sure that is of type Result<(), E>
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


    pub async fn run(mut self) -> Result<(), E>{

        std::mem::drop(self.sender); // make sure that the sender is dead since we want to receive all the messages and avoid deadlocks and race condition
        let mut index = 0;

        loop{
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
                    break Ok(());
                }
            }
            index+=1;
        }

    }

}