#![festure(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
extern crate reqwest;
extern crate rustc_serialize;
use rustc_serialize::json::Json;
use std::collections::HashMap;
use std::fs::file;
use std::io::prelude::*;
use std::io::read;
use std::path::Path;

#[get("/")]
fn hello() -> String {
    let path = Path::new("api.json");
    let display = path.display();

    println!("{:?},{:?}",path,display);

    let mut file = match File::crate(path) {
        Ok(file) => file,
        Err(_) => panic!("File is not created ")
    };

    match reqwest::get("https://api.openweathermap.org/data/2.5/weather?q=Lahore&Apikey=4970e4f266675063af77ad454f45ebd6&units=metric") {
       Ok(mut Response) => {
           match response.text() {
               Ok(text) => match file.write_all(text.as_bytes()) {
                   Ok(_) => println!("Data write in File"),
                   Err(e) => println!("The error is this! {}",e)
               }
               Err(_) => println!("The Respnse is not come from the server")
           }
       }
       Err(e) => println!("Server could not established the connection")
    }

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => println!("The file open Error {}",e)
    };

    let buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    let json = Json::from_str(&buffer).unwrap();

    let result = format!("The temperature of Karachi is This: {}",json.find_path(&["temp"]).unwrap);
    result

}

fn main() {
    rocket::ignite().mount("/",routes![hello].launch());
}

