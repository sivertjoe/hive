use super::util::*;
use super::*;
use seed::{self, prelude::*};

pub struct SelectedPiece {
    pub piece: Piece,
    pub old_square: Square,
    pub x: f32,
    pub y: f32,
}

impl SelectedPiece {
    pub fn node(&self, model: &Model) -> Node<crate::Msg> {
        let pos = (self.x, self.y);
        piece_to_node(model, &self.piece, pos)
    }
}

impl From<Hex> for SelectedPiece {
    fn from(mut hex: Hex) -> Self {
        let piece = hex.remove_top().unwrap();
        let old_square = hex.sq();

        let (x, y) = hex.to_pixels();

        Self {
            piece,
            old_square,
            x,
            y,
        }
    }
}
