use std::env;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config_file_path = &args[1];

    let config = config::parse(&config_file_path);

    println!("{:?}", config.file_path);
    println!("{:?}", config.mappings);
    println!("{:?}", config.mappings[0].regex);
    println!("{:?}", config.mappings[0].event);
    println!("{:?}", config.mappings[1].regex);
    println!("{:?}", config.mappings[1].event);
}
