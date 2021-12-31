use std::error::Error;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, Response};
use serde::{Serialize, Deserialize};
use dotenv::dotenv;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub username: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserApiResponse {
    pub data: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FollowerApiResponse {
    pub data: Vec<User>,
}

async fn req_get(url: String) -> Result<Response, Box<dyn Error>> {
    dotenv()?;
    let bearer_token = env::var("BEARER_TOKEN")?;
    let client = reqwest::Client::new();

    Ok(client.get(url)
        .header(AUTHORIZATION, format!("Bearer {}", bearer_token))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap()
    )
}

// Get user by username
pub async fn get_user(username: &str) -> Result<User, Box<dyn Error>> {
    let url = format!("https://api.twitter.com/2/users/by/username/{}", username);
    let resp = req_get(url).await?;

    match resp.status() {
        reqwest::StatusCode::OK => {
            match resp.json::<UserApiResponse>().await {
                Ok(parsed) => Ok(parsed.data),
                Err(_) => panic!("Uh oh, something went wrong"),
            }
        }
        _ => {
            panic!("Uh oh, something went wrong");
        }
    }
}

// Get user follower count
pub async fn get_follower_count(user_id: &String) -> Result<usize, Box<dyn Error>> {
    let url = format!("https://api.twitter.com/2/users/{}/followers", user_id);
    let resp = req_get(url).await?;

    match resp.status() {
        reqwest::StatusCode::OK => {
            match resp.json::<FollowerApiResponse>().await {
                Ok(parsed) => Ok(parsed.data.len()),
                Err(_) => panic!("Uh oh, something went wrong"),
            }
        }
        _ => {
            panic!("Uh oh, something went wrong");
        }
    }
}