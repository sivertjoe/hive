use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use shared::model::UserCredentials;

use crate::database::hash;

#[derive(Debug, Serialize, Deserialize)]
pub struct User
{
    pub _id:           ObjectId,
    pub name:          String,
    pub password_hash: String,
    pub create_games:  Vec<ObjectId>,
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
            _id: ObjectId::new(),
            name,
            password_hash: hash(&password),
            create_games: Vec::new(),
            active_games: Vec::new(),
        }
    }
}
