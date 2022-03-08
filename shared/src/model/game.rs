use std::collections::HashMap;

use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};


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

pub fn legal_moves(p: &Piece, board: &Board, board_pos: &Option<Square>) -> Vec<Square>
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


    match board.turns
    {
        // These first we _know_ and can be hardcoded
        0 => vec![(0, 0, 0)],
        1 => vec![(1, -1, 0), (-1, 1, 0), (1, 0, -1), (-1, 0, 1), (0, 1, -1), (0, -1, 1)],

        _ =>
        {
            /*if (board.turns == 7 || board.turns == 8) && !contains_queen()
            {
                todo!()
            }*/

            match board_pos
            {
                Some(_pos) => todo!(), // Moving piece on the board,
                None => legal_new_piece_moves(p, board),
            }
        },
    }
}

// Hmm, t-this can be simplified r-right?
fn legal_new_piece_moves(piece: &Piece, board: &Board) -> Vec<Square>
{
    // Good neighbors have only same color neighbors or none
    let good_neighbors = |sq: &Square| {
        neighbors(&sq).into_iter().all(|sq| match board.board.get(&sq)
        {
            None => true,
            Some(s) => s.piece.color == piece.color,
        })
    };

    let not_touching_other_color =
        //|sq: Square| board.board.get(&sq).map_or(true, |s| s.piece.color == piece.color);
        |sq: Square| board.board.get(&sq).is_none() && good_neighbors(&sq);

    board
        .board
        .iter()
        .filter_map(|(sq, bp)| {
            (bp.piece.color == piece.color).then(|| {
                neighbors(sq)
                    .into_iter()
                    .filter_map(|sq| not_touching_other_color(sq).then(|| sq))
            })
        })
        .flatten()
        .collect()
}


type Stack = Option<Box<BPiece>>;

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

        self.turns += 1;
    }
}


pub type Square = (isize, isize, isize);

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum BoardPiece
{
    Queen,
    Ant,
    Spider,
    Beetle,
    Grasshopper,
}


fn neighbors(sq: &Square) -> [Square; 6]
{
    const CUBE_DIR_VEC: [(isize, isize, isize); 6] =
        [(1, 0, -1), (1, -1, 0), (0, -1, 1), (-1, 0, 1), (-1, 1, 0), (0, 1, -1)];

    let mut iter = CUBE_DIR_VEC.into_iter().map(|d| (sq.0 + d.0, sq.1 + d.1, sq.2 + d.2));

    [
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    ]
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn test_get_correct_neighbors()
    {
        let same = |a: [Square; 6], b: [Square; 6]| a.iter().all(|a| b.contains(a));

        assert!(same(neighbors(&(0, 0, 0)), [
            (0, -1, 1),
            (0, 1, -1),
            (1, 0, -1),
            (-1, 0, 1),
            (1, -1, 0),
            (-1, 1, 0)
        ]));

        assert!(same(neighbors(&(0, -2, 2)), [
            (0, -3, 3),
            (1, -3, 2),
            (1, -2, 1),
            (0, -1, 1),
            (-1, -1, 2),
            (-1, -2, 3)
        ]));
    }
}
