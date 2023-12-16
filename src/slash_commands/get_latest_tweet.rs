use tokio::process::Command as TokioCommand;
use std::error::Error;

pub async fn get_latest_tweet() -> Result<String, Box<dyn Error>> {
    let script_path = "web_scraper/script.js";

    let output = TokioCommand::new("node")
        .arg(script_path)
        .output()
        .await?;

    if output.status.success() {
        let tweet_link = String::from_utf8_lossy(&output.stdout);
        Ok(tweet_link.to_string())
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, err)))
    }
}