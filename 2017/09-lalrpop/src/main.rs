extern crate failure;
//extern crate combine;

use std::collections::HashMap;

use std::io::Read;
use std::fs::File;

mod stream;

#[derive(Debug, Clone)]
pub enum Node {
    Group(Vec<Node>),
    Garbage(Vec<String>),
}

fn main() {
    run().unwrap();
}

fn run() -> Result<(), failure::Error> {

//    let mut map = HashMap::new();

    let mut s = String::new();
    File::open("data").and_then(|mut f| f.read_to_string(&mut s))?;

    match stream::parse_group(&s) {
        Ok(o) => println!("success: {:?}", o),
        Err(e) => eprintln!("error: {:?}", e)
    }

//    println!("biggest {:?}", map.values().map(|x| *x).max());
//    println!("biggerest {:?}", biggest);

    Ok(())

}