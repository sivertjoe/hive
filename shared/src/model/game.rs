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
    pub game_object_id: ObjectId,
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


#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
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

pub fn legal_moves(p: &Piece, board: &Board, board_pos: &Option<usize>) -> Vec<Square>
{
    let search_stack = |next: &Stack| match next
    {
        Some(bp) => bp.piece.r#type == BoardPiece::Queen && bp.piece.color == p.color,
        None => false,
    };

    let contains_queen = || {
        board
            .board
            .values()
            .position(|bp| {
                bp.piece.r#type == BoardPiece::Queen && bp.piece.color == p.color
                    || search_stack(&bp.next)
            })
            .is_some()
    };


    match board.board.len()
    {
        // These first we _know_ and can be hardcoded
        0 => vec![(0, 0, 0)],
        1 => vec![(1, -1, 0), (-1, 1, 0), (1, 0, -1), (-1, 0, 1), (0, 1, -1), (0, -1, 1)],

        _ =>
        {
            if (board.turns == 7 || board.turns == 8) && !contains_queen()
            {
                todo!()
            }

            match board_pos
            {
                Some(pos) => todo!(), // Moving piece on the board,
                None => legal_new_piece_moves(p, board),
            }
        },
    }
}

fn legal_new_piece_moves(piece: &Piece, board: &Board) -> Vec<Square>
{
    board
        .board
        .iter()
        .filter(|(_, bp)| bp.piece.color != piece.color)
        .filter(|(sq, _)| {
            // Filter piece without an untouched neighbour
            true
        });


    Vec::new()
}


type Stack = Option<Box<BPiece>>;

#[derive(Serialize, Deserialize)]
struct BPiece
{
    piece: Piece,
    next:  Stack,
}

impl BPiece
{
    fn new(piece: Piece) -> Self
    {
        Self {
            piece,
            next: None,
        }
    }

    fn with_bpiece(piece: Piece, bpiece: BPiece) -> Self
    {
        Self {
            piece,
            next: Some(Box::new(bpiece)),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Board
{
    board:     HashMap<Square, BPiece>,
    pub turns: usize,
}

impl Board
{
    fn new() -> Self
    {
        Self {
            board: HashMap::new(), turns: 0
        }
    }

    pub fn place_piece(&mut self, piece: Piece, sq: Square)
    {
        match self.board.remove(&sq)
        {
            Some(bpiece) =>
            {
                self.board.insert(sq, BPiece::with_bpiece(piece, bpiece));
            },
            None =>
            {
                self.board.insert(sq, BPiece::new(piece));
            },
        }
    }
}


pub type Square = (isize, isize, isize);

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum BoardPiece
{
    Queen,
    Ant,
    Spider,
    Beetle,
    Grasshopper,
}


#[cfg(test)]
mod test
{
    use super::*;
}
