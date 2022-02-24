use serde::{Deserialize, Serialize};
use shared::model::UserCredentials;

use crate::database::{hash, uuid};

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
