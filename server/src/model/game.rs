use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use shared::model::Board;

#[derive(Serialize, Deserialize)]
pub struct Game
{
    players:   [ObjectId; 2],
    complete:  bool,
    pub board: Board,
}

impl Game
{
    pub fn new(players: [ObjectId; 2]) -> Self
    {
        Self {
            players,
            complete: false,
            board: Board::default(),
        }
    }
}
