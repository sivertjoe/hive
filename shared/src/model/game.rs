use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::Uuid;


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
const BOARD_SIZE: u32 = 2 + 4 + 4 + 6 + 6;

#[derive(Serialize, Deserialize)]
pub struct Game
{
    players: [Uuid; 2],
    board:   Board,
}

#[derive(Serialize, Deserialize)]
enum BoardSquare
{
    Occupied((BoardPiece, Box<BoardSquare>)),
    Empty,
}

#[derive(Serialize, Deserialize)]
pub struct Board
{
    board: HashMap<Square, BoardPiece>,
}


pub type Square = u32;

#[derive(Serialize, Deserialize)]
enum BoardPiece
{
    Queen,
    Ant,
    Spider,
    Beetle,
    Grasshopper,
}

#[derive(Serialize, Deserialize)]
pub struct Move
{
    piece: BoardPiece,
    from:  Square,
    to:    Square,
}

pub trait Piece: Serialize + serde::de::DeserializeOwned
{
    fn available_moves(&self) -> Vec<Move>;
}
