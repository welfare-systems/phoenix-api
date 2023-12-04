// Copyright © 2023 Jonathan Vázquez
// All rights reserved.
use dotenv::dotenv;
use std::env;

pub struct AppConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub db_host: String,
    pub db_username: String,
    pub db_password: String,
    pub db_name: String,
}

impl AppConfig {
    pub fn load() -> Self {
        dotenv().ok();

        AppConfig {
            client_id: env::var("CLIENT_ID").expect("CLIENT_ID not found in .env"),
            client_secret: env::var("CLIENT_SECRET").expect("CLIENT_SECRET not found in .env"),
            redirect_uri: env::var("REDIRECT_URI").expect("REDIRECT_URI not found in .env"),
            db_host: env::var("DB_HOST").expect("DB_HOST not found in .env"),
            db_username: env::var("DB_USERNAME").expect("DB_USERNAME not found in .env"),
            db_password: env::var("DB_PASSWORD").expect("DB_PASSWORD not found in .env"),
            db_name: env::var("DB_NAME").expect("DB_NAME not found in .env"),
        }
    }
}
