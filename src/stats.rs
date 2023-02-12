use crate::Config;
use isahc::{prelude::*, Request};
use serde_json::{json, Value};
use std::{collections::HashMap, io::Read, time::Instant};

pub struct Stats {
    pub users: HashMap<String, String>,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            users: HashMap::new(),
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

        // Authentication with osu api
        let oauth = Request::post("https://osu.ppy.sh/oauth/token")
            .header("Content-Type", "application/json")
            .body(
                json!({
                    "client_id": config.client_id,
                    "client_secret": config.client_secret,
                    "grant_type": "client_credentials",
                    "scope": "public"
                })
                .to_string(),
            )
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

        // Fetching stats for each configured user
        for user_id in config.user_ids.iter() {
            let stats = Request::get(format!("https://osu.ppy.sh/api/v2/users/{}", user_id))
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", token))
                .body(())
                .unwrap();

            match stats.send().as_mut() {
                Ok(response) => {
                    let mut body = String::new();
                    response.body_mut().read_to_string(&mut body).unwrap();
                    self.users.insert(user_id.to_owned(), body);
                }

                Err(error) => {
                    panic!("Couldn't update stats: {error}");
                }
            };
        }

        println!("Updated stats!");
    }
}
