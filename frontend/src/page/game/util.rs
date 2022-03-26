use super::*;
use crate::page::game::*;
use seed::{self, prelude::*};
use shared::{model::game::*, ObjectId};
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

pub fn clear_red(model: &mut Model) {
    for hex in &mut model.gridv3 {
        hex.red = false;
    }
}

pub fn set_highlight(model: &mut Model, moves: Vec<Square>, val: bool) {
    for mov in moves {
        get_hex_from_square(model, mov).as_mut().unwrap().selected = val;
    }
}

pub fn get_move(model: &Model, sel: Piece, sq: Square, old_sq: Option<Square>) -> Option<Move> {
    let id: Result<ObjectId, _> = LocalStorage::get("id");
    let name: Result<String, _> = LocalStorage::get("name");

    if let (Ok(id), Ok(name), Some(game)) = (id, name, model.game.as_ref()) {
        if game.players.iter().any(|_name| &name == _name) {
            return Some(Move {
                piece: sel,
                player_id: id,
                game_id: game._id,
                old_sq,
                sq,
            });
        }
    }
    None
}

pub fn get_color(game: &GameResource) -> Option<Color> {
    LocalStorage::get("name").ok().and_then(|name: String| {
        game.players
            .iter()
            .position(|n| n == &name)
            .map(|i| match i {
                0 => Color::White,
                _ => Color::Black,
            })
    })
}

pub fn grid_from_board(model: &mut Model) {
    for (&sq, board_square) in model.game.as_ref().unwrap().board.board.iter() {
        for piece in &board_square.pieces {
            let hex = model.gridv3.iter_mut().find(|hex| hex.sq() == sq).unwrap();
            hex.place_piece(*piece);
        }
    }
}

pub fn get_move_color(model: &Model) -> Option<Color> {
    model.game.as_ref().map(|game| match game.board.turns % 2 {
        0 => Color::White,
        _ => Color::Black,
    })
}

pub fn legal_turn(model: &Model) -> bool {
    match (get_move_color(model), model.color) {
        (Some(c1), Some(c2)) => c1 == c2,
        _ => false,
    }
}

pub fn legal_piece(model: &Model) -> bool {
    match (model.piece.as_ref(), model.color) {
        (Some(ref sel), Some(ref color)) => sel.piece.color == *color,
        _ => false,
    }
}

pub fn game_complete(model: &Model) -> bool {
    model
        .game
        .as_ref()
        .map(|game| game.is_complete())
        .unwrap_or(false)
}
