use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use shared::model::Board;

use super::user::User;

#[derive(Serialize, Deserialize)]
pub struct Game
{
    players:   [Player; 2],
    complete:  bool,
    pub board: Board,
}

impl Game
{
    pub fn new(users: [User; 2]) -> Self
    {
        let get_players = |users: [User; 2]| {
            let [u1, u2] = users;
            [
                Player {
                    _id: u1._id, name: u1.name
                },
                Player {
                    _id: u2._id, name: u2.name
                },
            ]
        };

        Self {
            players: get_players(users), complete: false, board: Board::default()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Player
{
    _id:  ObjectId,
    name: String,
}
