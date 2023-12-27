use fs2::FileExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};

/*Currently, this only works on a windows server. Need to add functionality to work on linux*/

#[derive(Serialize, Deserialize, Default)]
pub struct Tokens {
    token_map: HashMap<String, String>,
}

impl Tokens {
    pub fn load() -> Result<Self, std::io::Error> {
        let file = File::open("tokens.dat")?;
        file.lock_exclusive()?;
        let tokens: Tokens = serde_json::from_reader(&file)?;
        file.unlock()?;
        Ok(tokens)
    }

    pub fn add_token(&mut self, user_id: String, token: String) {
        self.token_map.insert(user_id, token);
        self.save().unwrap();
    }

    pub fn find_token_by_user_id(&self, user_id: &str) -> Option<String> {
        self.token_map.get(user_id).cloned()
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
