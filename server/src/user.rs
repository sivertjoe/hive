use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use shared::{model::UserCredentials, Uuid};

use crate::database::{hash, uuid};

#[derive(Debug, Serialize, Deserialize)]
pub struct User
{
    pub name:          String,
    pub password_hash: String,
    pub uuid:          Uuid,
    pub create_games:  Vec<Uuid>,
    pub active_games:  Vec<ObjectId>,
}

impl User
{
    pub fn from_cred(cred: UserCredentials) -> Self
    {
        let UserCredentials {
            name,
            password,
        } = cred;

        Self {
            name,
            password_hash: hash(&password),
            uuid: uuid(),
            create_games: Vec::new(),
            active_games: Vec::new(),
        }
    }
}
