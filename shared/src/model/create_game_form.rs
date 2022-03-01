use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateGameFormResponse
{
    pub game:    ObjectId,
    pub creator: ObjectId,
    pub user:    ObjectId,
}


#[derive(Serialize, Deserialize)]
pub struct CreateGameChallenge
{
    pub name:    String,
    pub creator: ObjectId,
    pub _id:     ObjectId,
}

#[derive(Serialize, Deserialize)]
pub struct CreateGameChallengeBundle
{
    pub name:       String,
    pub creator_id: ObjectId,
    pub games:      Vec<ObjectId>,
}


#[derive(Serialize, Deserialize)]
pub struct AcceptGame
{
    pub object_id: String,
    pub game:      ObjectId,
}
