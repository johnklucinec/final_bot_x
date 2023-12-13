use tokio::process::Command as TokioCommand;


pub async fn get_latest_tweet() -> String {
  let script_path = "web_scraper/script.js";

  let output = TokioCommand::new("node")
      .arg(script_path)
      .output().await;

  // Handle the Result
  let output = match output {
      Ok(output) => output,
      Err(e) => {
          println!("Failed to execute command: {}", e);
          return String::new();
      }
  };

  let tweet_link = String::from_utf8_lossy(&output.stdout);

  tweet_link.to_string()
}
