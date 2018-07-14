use crossbeam;
use regex::Regex;

use config::{Config};

pub struct Dispatcher<'a> {
    config: &'a Config
}

impl<'a> Dispatcher<'a> {
    pub fn new(config: &Config) -> Dispatcher {
        let dispatcher = Dispatcher {
            config: config
        };
        dispatcher
    }

    pub fn handle(&self, log_line: &str) {
        for mapping in &self.config.mappings {
            let re = Regex::new(&mapping.regex).unwrap();
            if re.is_match(log_line) {
                crossbeam::scope(|scope| {
                    scope.spawn(move || {
                        &self.emit(&mapping.event);
                    });
                })
            }
        }
    }

    fn emit(&self, event: &str) {
        println!("Emitting event \"{}\"", event);
    }
}
