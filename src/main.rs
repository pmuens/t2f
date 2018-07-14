use std::env;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate chase;
extern crate regex;

mod config;
mod dispatcher;

use chase::Chaser;
use dispatcher::Dispatcher;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config_file_path = &args[1];

    let config = config::parse(&config_file_path);
    let dispatcher = Dispatcher::new(&config);
    let chaser = Chaser::new(&config.file_path);

    let (receiver, _) = chaser.run_channel().unwrap();

    loop {
        let log_line = &receiver.recv().unwrap().0;
        &dispatcher.handle(log_line);
    }
}
