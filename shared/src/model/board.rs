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

// const Y: usize = 2 * (3 * 3 * 2 * 2 * 1);

#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Board
{
    #[serde_as(as = "Vec<(_, _)>")]
    board: HashMap<Square, BoardSquare>,

    pub queens: [Option<Square>; 2],
    pub turns:  usize,
}

impl Board
{
    pub fn get(&self, sq: &Square) -> Option<&BoardSquare>
    {
        self.board.get(sq)
    }

    pub fn contains_key(&self, sq: &Square) -> bool
    {
        self.board.contains_key(sq)
    }

    pub fn len(&self) -> usize
    {
        self.board.len()
    }

    pub fn insert(&mut self, sq: Square, bs: BoardSquare)
    {
        self.board.insert(sq, bs);
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<Square, BoardSquare>
    {
        self.board.iter()
    }

    pub fn values(&self) -> std::collections::hash_map::Values<Square, BoardSquare>
    {
        self.board.values()
    }

    pub fn remove(&mut self, sq: Square)
    {
        self.board.remove(&sq);
    }

    pub fn from_iter<I>(&mut self, iter: I)
    where
        I: Iterator<Item = (Square, BoardSquare)>,
    {
        self.board = HashMap::from_iter(iter);
    }

    pub fn empty_square(&self, sq: &Square) -> bool
    {
        match self.get(sq)
        {
            Some(bs) => bs.pieces.is_empty(),
            None => true,
        }
    }

    pub fn play_move(&mut self, r#move: Move)
    {
        self.play_move_(r#move.into());
    }

    pub fn play_move_(&mut self, r#move: InternalMove)
    {
        self.place_piece(r#move.piece, r#move.sq, r#move.old_sq);
    }

    pub fn unplay_move(&mut self, r#move: InternalMove)
    {
        let bs = self.board.get_mut(&r#move.sq).unwrap();
        let p = bs.remove_piece().unwrap();

        if let Some(old) = r#move.old_sq
        {
            if let Some(ref mut bs) = self.board.get_mut(&old)
            {
                bs.place_piece(p);
            }
            else
            {
                self.board.insert(old, BoardSquare::new(p));
            }
        }
    }

    pub fn un_play_from_to(&mut self, from: Square, to: Square)
    {
        if let Some(bs) = self.board.get_mut(&to)
        {
            let p = unsafe
            {
                bs.remove_piece().unwrap_unchecked()
            };
            self.board.entry(from).and_modify(|bs| bs.place_piece(p));
        }
    }

    pub fn play_from_to(&mut self, from: Square, to: Square)
    {
        let bs = self.board.get_mut(&from).unwrap();
        let old = bs.remove_piece().unwrap();
        if bs.pieces.is_empty()
        {
            self.board.remove(&from).unwrap();
        }

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
