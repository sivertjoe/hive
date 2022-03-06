use std::collections::HashMap;

use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

/*
 * According to the rules: https://www.ultraboardgames.com/hive/game-rules.php
 * there are:
 * 2 Queen bees
 * 4 Spiders
 * 4 Beetles
 * 6 Grasshoppers
 * 6 Ants

    Thus, the maximum board length in any direction is:
*/
const BOARD_SIZE: usize = 2 + 4 + 4 + 6 + 6;

#[derive(Serialize, Deserialize)]
pub struct OnGoingGame
{
    pub game_object_id: String,
    pub players:        [String; 2],
}

#[derive(Serialize, Deserialize)]
pub struct Game
{
    pub players: [ObjectId; 2],
    pub board:   Board,
}

impl Game
{
    pub fn new(players: [ObjectId; 2]) -> Self
    {
        Self {
            players,
            board: Board::new(),
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum Color
{
    White,
    Black,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Piece
{
    pub r#type: BoardPiece,
    pub color:  Color,
}

pub fn legal_moves(p: &Piece, board: &Board) -> Vec<Square>
{
    if board.board.len() == 0 { vec![0] } else { todo!() }
}

#[derive(Serialize, Deserialize)]
enum BoardSquare
{
    Occupied((Piece, Box<BoardSquare>)),
    Empty,
}

#[derive(Serialize, Deserialize)]
pub struct Board
{
    board: HashMap<Square, BoardPiece>,
}

impl Board
{
    fn new() -> Self
    {
        Self {
            board: HashMap::with_capacity(BOARD_SIZE)
        }
    }
}


pub type Square = u32;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum BoardPiece
{
    Queen,
    Ant,
    Spider,
    Beetle,
    Grasshopper,
}
