use std::env;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate chase;

mod config;

use chase::Chaser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config_file_path = &args[1];

    let config = config::parse(&config_file_path);
    let chaser = Chaser::new(&config.file_path);

    let (receiver, _) = chaser.run_channel().unwrap();
    loop {
        let log_line = &receiver.recv().unwrap().0;
        println!("{}", log_line);
    }
}
