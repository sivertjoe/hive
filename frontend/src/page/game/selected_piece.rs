use super::*;
use seed::{self, prelude::*};

pub struct SelectedPiece {
    pub piece: Piece,
    pub old_square: Square,
    pub x: f32,
    pub y: f32,
}

impl SelectedPiece {
    pub fn node(&self) -> Node<crate::Msg> {
        let (x, y) = (self.x, self.y);

        let fill = piece_color(self.piece.r#type, self.piece.color);

        r#use![attrs! {
            At::Href => "#pod",
            At::Transform => format!("translate({x}, {y})"),
            At::Fill => fill,
            At::Stroke => "gold",
            At::Opacity => "1.0",
        },]
    }
}

impl From<Hex> for SelectedPiece {
    fn from(mut hex: Hex) -> Self {
        let piece = hex.remove_top().unwrap();
        let old_square = hex.sq();
        let x = hex._x;
        let y = hex._y;

        Self {
            piece,
            old_square,
            x,
            y,
        }
    }
}
