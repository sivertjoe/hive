use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User
{
    name:          String,
    password_hash: String,
    uuid:          String,
}

impl User
{
    pub fn new(name: String, password: String) -> Self
    {
        Self {
            name,
            password_hash: hash(password.as_str()),
            uuid: uuid(),
        }
    }
}

fn uuid() -> String
{
    format!("{}", uuid::Uuid::new_v4())
}

fn hash(word: &str) -> String
{
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(word);
    let result = hasher.finalize();
    format!("{result:x}")
}
