use serde::{Deserialize, Serialize};
use shared::model::UserCredentials;

#[derive(Debug, Serialize, Deserialize)]
pub struct User
{
    pub name:          String,
    pub password_hash: String,
    pub uuid:          String,
}

impl User
{
    pub fn from_cred(cred: UserCredentials) -> Self
    {
        let name = cred.name;
        let password = cred.password;
        Self::new(name, password)
    }

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
