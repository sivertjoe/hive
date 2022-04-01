use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::model::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BoardSquare
{
    pub pieces: Vec<Piece>,
}

impl BoardSquare
{
    pub fn new(piece: Piece) -> Self
    {
        Self {
            pieces: vec![piece]
        }
    }

    pub fn place_piece(&mut self, piece: Piece)
    {
        self.pieces.push(piece);
    }

    pub fn remove_piece(&mut self) -> Option<Piece>
    {
        self.pieces.pop()
    }

    pub fn top(&self) -> &Piece
    {
        self.pieces.last().unwrap()
    }
}

#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Board
{
    #[serde_as(as = "Vec<(_, _)>")]
    pub board: HashMap<Square, BoardSquare>,

    pub queens: [Option<Square>; 2],
    pub turns:  usize,
}

impl Board
{
    pub fn play_move(&mut self, r#move: Move)
    {
        // Have logic to check for legal move here
        self.place_piece(r#move.piece, r#move.sq, r#move.old_sq);
    }

    pub fn un_play_from_to(&mut self, from: Square, to: Square)
    {
        if let Some(bs) = self.board.get_mut(&to)
        {
            let p = bs.remove_piece().unwrap();
            self.board.entry(from).and_modify(|bs| bs.place_piece(p));
        }
    }

    pub fn play_from_to(&mut self, from: Square, to: Square)
    {
        let bs = self.board.get_mut(&from).unwrap();
        let old = bs.remove_piece().unwrap();

        self.board
            .entry(to)
            .and_modify(|bs| bs.place_piece(old))
            .or_insert_with(|| BoardSquare::new(old));
    }

    pub fn place_piece(&mut self, piece: Piece, sq: Square, old: Option<Square>)
    {
        if piece.r#type == BoardPiece::Queen
        {
            let idx = piece.color as usize;
            self.queens[idx] = Some(sq);
        }

        self.board
            .entry(sq)
            .and_modify(|bs| bs.place_piece(piece))
            .or_insert_with(|| BoardSquare::new(piece));

        if let Some(sq) = old
        {
            let bs = self.board.get_mut(&sq).unwrap();
            bs.remove_piece();
            if bs.pieces.is_empty()
            {
                self.board.remove(&sq).unwrap();
            }
        }

        self.turns += 1;
    }

    pub fn is_complete(&self) -> bool
    {
        self.queens.iter().any(|queen| match queen
        {
            Some(sq) =>
            {
                crate::r#move::neighbors(sq).into_iter().all(|sq| self.board.contains_key(&sq))
            },
            None => false,
        })
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum BoardPiece
{
    Queen,
    Ant,
    Spider,
    Beetle,
    Grasshopper,
}

impl From<String> for BoardPiece
{
    fn from(s: String) -> Self
    {
        use BoardPiece::*;
        match s.as_str()
        {
            "Queen" => Queen,
            "Ant" => Ant,
            "Spider" => Spider,
            "Beetle" => Beetle,
            "Grasshopper" => Grasshopper,
            _ => unreachable!(),
        }
    }
}
