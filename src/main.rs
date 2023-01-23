use serde::Deserialize;
use std::time::{Instant, Duration};
use tiny_http::{Header, Method, Response, Server};

mod stats;
use stats::Stats;

#[derive(Debug, Deserialize)]
pub struct Config {
    bind: String,
    pub client_id: String,
    pub secret: String,
    pub update_interval_minutes: u64,
    pub user_id: String,
}

fn main() {
    let json_file = std::fs::read_to_string("./config.json").expect("Couldn't read config.json");
    let config: Config = serde_json::from_str(&json_file).expect("Couldn't parse config.json");

    let mut last_update = Instant::now() - Duration::from_secs(config.update_interval_minutes * 60);
    let mut stats = Stats::default();
    stats.update(&mut last_update, &config);

    let server = Server::http(config.bind.as_str()).unwrap();
    println!("Running osu-microapi on {}", config.bind.as_str());

    for request in server.incoming_requests() {
        match request.method() {
            Method::Get => {
                if request.url() == "/" {
                    stats.update(&mut last_update, &config);

                    request
                        .respond(
                            Response::from_string(&stats.value)
                                .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap()),
                        )
                        .unwrap();

                    continue;
                }
            },
            _ => {}
        }

        request.respond(Response::empty(404)).unwrap();
    }
}
