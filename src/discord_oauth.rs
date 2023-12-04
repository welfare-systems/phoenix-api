// Copyright © 2023 Jonathan Vázquez
// All rights reserved.
use reqwest::Client;

#[derive(Debug)]
pub enum DiscordOAuthError {
    ReqwestError(reqwest::Error),
    JsonError(String),
}

impl From<reqwest::Error> for DiscordOAuthError {
    fn from(err: reqwest::Error) -> Self {
        DiscordOAuthError::ReqwestError(err)
    }
}

impl From<serde_json::Error> for DiscordOAuthError {
    fn from(err: serde_json::Error) -> Self {
        DiscordOAuthError::JsonError(err.to_string())
    }
}

pub struct DiscordOAuth {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
}

impl DiscordOAuth {
    pub fn new(client_id: &str, client_secret: &str, redirect_uri: &str) -> Self {
        DiscordOAuth {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            redirect_uri: redirect_uri.to_string(),
        }
    }

    pub fn get_authorization_url(&self) -> String {
        format!(
            "https://discord.com/api/oauth2/authorize?client_id={}&redirect_uri={}&response_type=code&scope=identify",
            self.client_id, self.redirect_uri
        )
    }

    pub async fn exchange_code_for_token(&self, code: &str) -> Result<String, DiscordOAuthError> {
        // Discord token endpoint URL
        let token_url = "https://discord.com/api/v10/oauth2/token";

        // Create a client for making HTTP requests
        let client = Client::new();

        // Prepare the request parameters
        let params = [
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
            ("grant_type", &"authorization_code".to_string()),
            ("code", &code.to_string()),
            ("redirect_uri", &self.redirect_uri),
            ("scope", &"identify".to_string()),
        ];

        let response = client
            .post(token_url)
            .form(&params)
            .send()
            .await
            .map_err(DiscordOAuthError::from)?;

        if response.status().is_success() {
            // Parse the response JSON to extract the access token
            let response_json: serde_json::Value = response.json().await?;
            let access_token = response_json["access_token"].as_str().unwrap_or_default();
            Ok(access_token.to_string())
        } else {
            // Log the error response for debugging
            let error_response = response.text().await?;
            eprintln!("Discord Token Exchange Error: {}", error_response);
            return Err(DiscordOAuthError::JsonError(
                "Access token not found in response".to_string(),
            ));
        }
    }
}
