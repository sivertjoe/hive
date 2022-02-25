use serde::{Deserialize, Serialize};

use crate::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateGameForm
{
    pub creator: Uuid,
}


#[derive(Serialize, Deserialize)]
pub struct CreateGameChallenge
{
    pub name: String,
    pub uuid: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateGameChallengeBundle
{
    pub name:  String,
    pub games: Vec<Uuid>,
}
