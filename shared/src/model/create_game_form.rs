use serde::{Deserialize, Serialize};

use crate::Uuid;

#[derive(Serialize, Deserialize)]
pub struct CreateGameForm
{
    pub creator: Uuid,
}


#[derive(Serialize, Deserialize)]
pub struct CreateGameChallenge
{
    pub creator: String,
    pub uuid:    String,
}
