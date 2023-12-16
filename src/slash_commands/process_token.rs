use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Write, Read};
use fs2::FileExt;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Tokens {
    token_map: HashMap<String, String>,
}

impl Tokens {
    pub fn load() -> Result<Self, std::io::Error> {
        let mut file = File::open("tokens.dat")?;
        file.lock_exclusive()?;
        let tokens: Tokens = serde_json::from_reader(&file)?;
        file.unlock()?;
        Ok(tokens)
    }

    pub fn add_token(&mut self, user_id: String, token: String) {
        self.token_map.insert(user_id, token);
        self.save().unwrap();
    }
  

    fn find_token_by_user_id(&self, user_id: &str) -> Option<String> {
        self.token_map.get(user_id).map(|v| v.clone())
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open("tokens.dat")?;
        file.lock_exclusive()?;
        serde_json::to_writer(&file, self)?;
        file.unlock()?;
        Ok(())
    }
}

fn manipulate_tokens() -> Result<(), std::io::Error> {
    let mut tokens = Tokens::load().unwrap_or_default();

    tokens.add_token("User1".to_string(), "Token1".to_string());

    let token = tokens.find_token_by_user_id("User1");
    println!("Found token: {:?}", token);

    Ok(())
}
