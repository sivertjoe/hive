use serde::{Deserialize, Serialize};

use crate::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateGameForm
{
    pub creator: Uuid,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateGameFormResponse
{
    pub game:    Uuid,
    pub creator: Uuid,
    pub user:    Uuid,
}


#[derive(Serialize, Deserialize)]
pub struct CreateGameChallenge
{
    pub name:    String,
    pub creator: Uuid,
    pub uuid:    Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct CreateGameChallengeBundle
{
    pub name:  String,
    pub uuid:  Uuid,
    pub games: Vec<Uuid>,
}


#[derive(Serialize, Deserialize)]
pub struct AcceptGame
{
    pub object_id: String,
    pub game:      Uuid,
}
