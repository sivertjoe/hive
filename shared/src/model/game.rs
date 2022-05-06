use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::model::board::*;

pub type Square = (isize, isize, isize);
type Name = String;


#[derive(Serialize, Deserialize, Clone)]
pub struct Move
{
    pub player_id: ObjectId,
    pub game_id:   ObjectId,
    pub piece:     Piece,
    pub sq:        Square,
    pub old_sq:    Option<Square>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct InternalMove
{
    pub piece:  Piece,
    pub sq:     Square,
    pub old_sq: Option<Square>,
}

impl From<Move> for InternalMove
{
    fn from(r#move: Move) -> Self
    {
        Self {
            piece: r#move.piece, sq: r#move.sq, old_sq: r#move.old_sq
        }
    }
}


#[derive(Serialize, Deserialize)]
pub struct OnGoingGame
{
    pub game_object_id: ObjectId,
    pub players:        [Name; 2],
}


#[derive(Serialize, Deserialize)]
pub struct GameResource
{
    pub _id:       ObjectId,
    pub players:   [Name; 2],
    pub board:     Board,
    pub move_list: Vec<InternalMove>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Color
{
    White,
    Black,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Piece
{
    pub r#type: BoardPiece,
    pub color:  Color,
}

impl Piece
{
    #[allow(dead_code)]
    pub fn new(r#type: BoardPiece, color: Color) -> Self
    {
        Self {
            color,
            r#type,
        }
    }
}

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
    pub object_id: ObjectId,
    pub game:      ObjectId,
}
