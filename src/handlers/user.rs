// Copyright © 2023 Jonathan Vázquez
// All rights reserved.
use actix_web::{HttpRequest, HttpResponse, Responder};
use reqwest;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct DiscordUser {
    id: String,
    username: String,
    discriminator: String,
    avatar: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct Guild {
    id: String,
    name: String,
    permissions: String, // Discord permissions are represented as integers
}

#[derive(serde::Serialize, Debug)]
struct UserInfoResponse {
    user: DiscordUser,
    guilds: Vec<Guild>,
}

pub async fn get_user_info(request: HttpRequest) -> impl Responder {
    // Access the access_token from the cookie
    let access_token_cookie = request.cookie("access_token");
    if let Some(cookie) = access_token_cookie {
        let access_token = cookie.value();

        // Fetch user information from Discord API
        let discord_user: DiscordUser = match fetch_discord_user(&access_token).await {
            Ok(user) => user,
            Err(err) => {
                eprintln!("Error fetching Discord user: {:?}", err);
                return HttpResponse::InternalServerError().finish();
            }
        };

        // Fetch guild memberships and permissions from Discord API
        let guilds: Vec<Guild> = match fetch_user_guilds(&access_token).await {
            Ok(guilds) => guilds,
            Err(err) => {
                eprintln!("Error fetching user guilds: {:?}", err);
                return HttpResponse::InternalServerError().finish();
            }
        };

        // Construct the response
        let response = UserInfoResponse {
            user: discord_user,
            guilds,
        };

        HttpResponse::Ok().json(response)
    } else {
        HttpResponse::Unauthorized().body("Not authenticated!")
    }
}

async fn fetch_discord_user(access_token: &str) -> Result<DiscordUser, reqwest::Error> {
    // Make a request to the Discord API to fetch user information
    let discord_user_url = "https://discord.com/api/v10/users/@me";
    let client = reqwest::Client::new();
    let response = client
        .get(discord_user_url)
        .bearer_auth(access_token)
        .send()
        .await?;
    let discord_user: DiscordUser = response.json().await?;
    Ok(discord_user)
}

async fn fetch_user_guilds(access_token: &str) -> Result<Vec<Guild>, reqwest::Error> {
    // Make a request to the Discord API to fetch user guilds
    let user_guilds_url = "https://discord.com/api/v10/users/@me/guilds";
    let client = reqwest::Client::new();
    let response = client
        .get(user_guilds_url)
        .bearer_auth(access_token)
        .send()
        .await?;
    let guilds: Vec<Guild> = response.json().await?;
    Ok(guilds)
}
