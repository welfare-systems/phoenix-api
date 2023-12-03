// Copyright © 2023 Jonathan Vázquez
// All rights reserved.
use crate::discord_oauth::DiscordOAuth;
use actix_web::cookie::time as cookie_time;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use config::AppConfig;
use dotenv::dotenv;

mod config;
mod discord_oauth;

#[derive(Debug, serde::Deserialize)]
struct CallbackParams {
    code: String,
}

async fn login() -> impl Responder {
    // Env variables
    let app_config = AppConfig::load();

    // Redirect users to Discord for authentication
    let discord_oauth = DiscordOAuth::new(
        &app_config.client_id,
        &app_config.client_secret,
        &app_config.redirect_uri,
    );
    let authorization_url = discord_oauth.get_authorization_url();
    HttpResponse::Found()
        .append_header(("Location", authorization_url))
        .finish()
}

async fn callback(query: web::Query<CallbackParams>) -> impl Responder {
    // Env variables
    let app_config = AppConfig::load();

    // Handle the callback from Discord
    let discord_oauth = DiscordOAuth::new(
        &app_config.client_id,
        &app_config.client_secret,
        &app_config.redirect_uri,
    );

    match discord_oauth.exchange_code_for_token(&query.code).await {
        Ok(access_token) => {
            // Save the access_token in a cookie
            let mut response = HttpResponse::Found()
                .append_header(("Location", "/authenticated")) // Redirect to an authenticated route
                .finish();

            let cookie = Cookie::build("access_token", access_token)
                .path("/")
                .secure(true) // Set to true in production when using HTTPS
                .same_site(SameSite::None)
                .http_only(true)
                .max_age(cookie_time::Duration::days(1)) // Adjust the expiration time as needed
                .finish();

            let _ = response.add_cookie(&cookie);
            response
        }
        Err(_) => HttpResponse::Unauthorized().body("Authentication failed!"),
    }
}

async fn authenticated(request: HttpRequest) -> impl Responder {
    // Log the cookies to inspect them
    println!("Cookies: {:?}", request.cookies());

    // Retrieve the access token from the cookie
    if let Some(cookie) = request.cookie("access_token") {
        let access_token = cookie.value().to_string();
        println!("Access token: {}", access_token);
        return HttpResponse::Ok()
            .body(format!("Authenticated with access token: {}", access_token));
    } else {
        println!("No access token found in the cookie");
        HttpResponse::Unauthorized().body("Not authenticated!")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .route("/login", web::get().to(login))
            .route("/callback", web::get().to(callback))
            .route("/authenticated", web::get().to(authenticated))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
