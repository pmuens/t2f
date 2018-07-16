use crossbeam;
use tokio;
use regex::Regex;
use hyper_tls::HttpsConnector;
use futures::{future, Future};
use hyper::{Client, Uri, Request, Body, Method, header};

use config::{Config, Mapping};

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
                        &self.emit(log_line, &mapping);
                    });
                })
            }
        }
    }

    fn emit(&self, log_line: &str, mapping: &Mapping) {
        let json = json!({
            "data": &log_line,
            "regex": &mapping.regex
        }).to_string();

        let uri: Uri = self.config.event_gateway.url.parse().unwrap();
        let mut req = Request::new(Body::from(json));
        *req.method_mut() = Method::POST;
        *req.uri_mut() = uri.clone();
        let auth = format!("bearer {}", self.config.event_gateway.access_key);
        req.headers_mut().insert("Authorization", header::HeaderValue::from_str(&auth).unwrap());
        req.headers_mut().insert("content-type", header::HeaderValue::from_str("application/json").unwrap());
        req.headers_mut().insert("event", header::HeaderValue::from_str(&mapping.event).unwrap());

        println!("Emitting event \"{}\"", mapping.event);
        tokio::run(future::lazy(|| {
            let https = HttpsConnector::new(4).unwrap();
            let client = Client::builder()
                .build::<_, Body>(https);
            client.request(req)
              .map(|_res| {})
              .map_err(|e| println!("Request error: {}", e))
        }));
    }
}
