use std::{time::Instant, io::Read};
use crate::Config;
use serde_json::{json, Value};
use isahc::{prelude::*, Request};

pub struct Stats {
    pub value: String,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            value: "{\"progress\": \"fetching data\"}".to_string()
        }
    }
}

impl Stats {
    pub fn update(&mut self, last_update: &mut Instant, config: &Config) {
        if last_update.elapsed().as_secs() < config.update_interval_minutes * 60 {
            return;
        }
        *last_update = Instant::now();

        println!("Updating stats...");

        let oauth = Request::post("https://osu.ppy.sh/oauth/token")
            .header("Content-Type", "application/json")
            .body(json!({
                "client_id": config.client_id,
                "client_secret": config.secret,
                "grant_type": "client_credentials",
                "scope": "public"
            }).to_string())
            .unwrap();

        let token = match oauth.send().as_mut() {
            Ok(response) => {
                let mut body = String::new();
                response.body_mut().read_to_string(&mut body).unwrap();

                let authorization: Value = serde_json::from_str(&body).unwrap();
                authorization["access_token"].as_str().unwrap().to_owned()
            }

            Err(error) => {
                panic!("Error: {error}");
            }
        };

        let stats = Request::get(format!("https://osu.ppy.sh/api/v2/users/{}", config.user_id))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", token))
            .body(())
            .unwrap();

        match stats.send().as_mut() {
            Ok(response) => {
                let mut body = String::new();
                response.body_mut().read_to_string(&mut body).unwrap();

                let stats: Value = serde_json::from_str(&body).unwrap();
                self.value = serde_json::to_string(&stats).unwrap();

                println!("Updated!");
            }

            Err(error) => {
                panic!("Error: {error}");
            }
        };

    }
}