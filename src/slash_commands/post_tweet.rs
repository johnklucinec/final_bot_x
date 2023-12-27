use reqwest::header::AUTHORIZATION;
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
pub struct Media {
    media_ids: Vec<String>,
}

#[derive(Serialize)]
pub struct Reply {
    in_reply_to_tweet_id: String,
    exclude_reply_user_ids: Vec<String>,
}

#[derive(Serialize)]
pub struct TweetParams {
    pub(crate) text: Option<String>,
    pub(crate) media: Option<Media>,
    pub(crate) reply: Option<Reply>,
}

pub async fn send_tweets(
    params_vec: Vec<TweetParams>,
    bearer_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let request_url = "https://api.twitter.com/2/tweets";

    for params in params_vec {
        let response = client
            .post(request_url)
            .header(AUTHORIZATION, format!("Bearer {}", bearer_token))
            .header(CONTENT_TYPE, "application/json")
            .body(json!(params).to_string())
            .send()
            .await?;

        if response.status().is_success() {
            println!("Tweet sent successfully!");
        } else {
            println!("Failed to send tweet: {}", response.text().await?);
        }
    }

    Ok(())
}

pub async fn send_tweet(
    params: TweetParams,
    bearer_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let request_url = "https://api.twitter.com/2/tweets";

    let response = client
        .post(request_url)
        .header(AUTHORIZATION, format!("Bearer {}", bearer_token))
        .header(CONTENT_TYPE, "application/json")
        .body(json!(params).to_string())
        .send()
        .await?;

    if response.status().is_success() {
        println!("Tweet sent successfully!");
    } else {
        println!("Failed to send tweet: {}", response.text().await?);
    }

    Ok(())
}
