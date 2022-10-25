





use std::fmt;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Serialize, Deserialize};






pub struct Proxy; // todo ➔ connect to different vpn every 30 seconds to avoid hit by blocking, use a socks5 proxy list





 

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
pub struct TheDosError{
    pub code: u16,
    pub msg: String,
}


impl TheDosError{

    pub async fn show(){
        (
            || async move{
                34
            }
        )().await;
    }

}



impl fmt::Display for TheDosError{ // implementing the formatter Display trait for the TheDosError struct to log every instance of it in a specific format to the console using println!() macro
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{ // self refers to the instance of the TheDosError
        f.write_fmt( // write the TheDoError args into a formatter which is `f` in here
            format_args!( // using format_args!() macro to print the following to the console when we log an instance of the TheDosError 
                "ERROR:{:#?}",
                &serde_json::to_string(self).map_err(|_| fmt::Error).unwrap() // return format error if there was any error in mapping each field
            )   
        )
    }

}