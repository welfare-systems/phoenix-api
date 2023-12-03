// Copyright © 2023 Jonathan Vázquez
// All rights reserved.
use dotenv::dotenv;
use std::env;

pub struct AppConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

impl AppConfig {
    pub fn load() -> Self {
        dotenv().ok();

        AppConfig {
            client_id: env::var("CLIENT_ID").expect("CLIENT_ID not found in .env"),
            client_secret: env::var("CLIENT_SECRET").expect("CLIENT_SECRET not found in .env"),
            redirect_uri: env::var("REDIRECT_URI").expect("REDIRECT_URI not found in .env"),
        }
    }
}
