use crate::page::game::*;
use seed::{self, prelude::*, *};
use shared::model::game::*;
use web_sys::MouseEvent;

pub fn get_board_mut(model: &mut Model) -> Option<&mut Board> {
    model.game.as_mut().map(|game| &mut game.board)
}
pub fn get_board(model: &Model) -> Option<&Board> {
    model.game.as_ref().map(|game| &game.board)
}

pub fn get_hex_from_square(model: &mut Model, sq: Square) -> Option<&mut Hex> {
    model.gridv3.iter_mut().find(|hex| hex.sq() == sq)
}

pub fn place_piece(model: &mut Model, piece: Piece, sq: Square) {
    if let Some(hex) = get_hex_from_square(model, sq) {
        hex.place_piece(piece);
    }
}

pub fn get_mouse_pos(model: &Model, mm: &MouseEvent) -> (f32, f32) {
    let ctm = model.svg.get().unwrap().get_screen_ctm().unwrap();
    let (x, y) = (mm.client_x(), mm.client_y());
    let (x, y) = (x as f32, y as f32);

    ((x - ctm.e()) / ctm.a(), (y - ctm.f()) / ctm.d())
}

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

pub fn piece_color(b: BoardPiece, color: Color) -> &'static str {
    if color == Color::White {
        match b {
            BoardPiece::Queen => "gold",
            BoardPiece::Ant => "blue",
            BoardPiece::Spider => "peru",
            BoardPiece::Grasshopper => "palegreen",
            BoardPiece::Beetle => "rebeccapurple",
        }
    } else {
        match b {
            BoardPiece::Queen => "DarkGoldenRod",
            BoardPiece::Ant => "MidnightBlue",
            BoardPiece::Spider => "brown",
            BoardPiece::Grasshopper => "green",
            BoardPiece::Beetle => "indigo",
        }
    }
}

pub fn get_piece_from_square_mut(model: &mut Model, sq: Square) -> Option<&mut Hex> {
    model
        .gridv3
        .iter_mut()
        .find(|hex| hex.sq() == sq && hex.top().is_some())
}

pub fn legal_move(model: &Model, sq: Square) -> bool {
    model
        .gridv3
        .iter()
        .find(|hex| hex.sq() == sq)
        .map(|hex| hex.selected)
        .unwrap_or(false)
}

pub fn place_piece_back(model: &mut Model, sel: SelectedPiece) {
    if let Some(old) = model
        .gridv3
        .iter_mut()
        .find(|hex| hex.sq() == sel.old_square)
    {
        old.place_piece(sel.piece);
    }
}

pub fn clear_highlighs(model: &mut Model) {
    for hex in &mut model.gridv3 {
        hex.selected = false;
    }
}

pub fn set_highlight(model: &mut Model, moves: Vec<Square>, val: bool) {
    for mov in moves {
        get_hex_from_square(model, mov).as_mut().unwrap().selected = val;
    }
}