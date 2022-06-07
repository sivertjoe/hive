use super::*;
use crate::page::game::*;
use seed::{self, prelude::*};
use shared::{model::*, ObjectId};
use web_sys::MouseEvent;

pub const RATIO: f32 = 1.1547005;

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

pub fn play_move(model: &mut Model, r#move: Move) {
    place_piece(model, r#move.piece, r#move.sq);
    get_board_mut(model).unwrap().play_move(r#move.clone());

    clear_yellow(&mut model.gridv3);

    if let Some(hex) = model.gridv3.iter_mut().find(|hex| hex.sq() == r#move.sq) {
        hex.yellow = true;
    }

    if let Some(old) = r#move.old_sq.as_ref() {
        if let Some(hex) = model.gridv3.iter_mut().find(|hex| hex.sq() == *old) {
            hex.yellow = true;
        }
    }

    model.game.as_mut().unwrap().move_list.push(r#move.into());
}

pub fn get_mouse_pos(model: &Model, mm: &MouseEvent) -> (f32, f32) {
    let ctm = model.svg.get().unwrap().get_screen_ctm().unwrap();
    let (x, y) = (mm.client_x(), mm.client_y());
    let (x, y) = (x as f32, y as f32);

    ((x - ctm.e()) / ctm.a(), (y - ctm.f()) / ctm.d())
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
        hex.selected_piece = false;
    }
}

#[allow(dead_code)]
pub fn clear_red(model: &mut Model) {
    for hex in &mut model.gridv3 {
        hex.red = false;
    }
}

pub fn set_highlight(model: &mut Model, val: bool) {
    if let Some(moves) = &model.legal_moves_cache {
        for mov in moves {
            //get_hex_from_square(model, *mov).as_mut().unwrap().selected = val;
            model
                .gridv3
                .iter_mut()
                .find(|hex| hex.sq() == *mov)
                .unwrap()
                .selected = val;
        }
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
    let board = &model.game.as_ref().unwrap().board;
    let mut grid = &mut model.gridv3;

    grid_from_board_(&mut grid, &board);
}

pub fn grid_from_board_(grid: &mut Vec<Hex>, board: &Board) {
    for hex in grid.iter_mut() {
        hex.pieces.clear();
    }

    for (&sq, board_square) in board.iter() {
        for piece in &board_square.pieces {
            let hex = grid.iter_mut().find(|hex| hex.sq() == sq).unwrap();
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
        (Some(sel), Some(ref color)) => sel.piece.color == *color,
        _ => false,
    }
}

pub fn game_complete(model: &Model) -> bool {
    model
        .game
        .as_ref()
        .map(|game| game.board.is_complete())
        .unwrap_or(false)
}

pub fn just_my_move(model: &Model, r#move: &Move) -> bool {
    model
        .color
        .as_ref()
        .map(|color| *color == r#move.piece.color)
        .unwrap_or(false)
}

pub fn sq_radius(sq: Square) -> usize {
    [sq.0.abs(), sq.1.abs(), sq.2.abs()]
        .into_iter()
        .max()
        .unwrap() as usize
        + 1
}

pub fn get_radius(model: &Model) -> usize {
    model
        .game
        .as_ref()
        .unwrap()
        .board
        .iter()
        .flat_map(|(sq, _)| [sq.0.abs(), sq.1.abs(), sq.2.abs()])
        .max()
        .unwrap_or(0) as usize
        + 1
}

// Nothing to see here 🧙
pub fn get_piece_dim(_model: &Model) -> (f32, f32) {
    let h = 50.0;

    (h * RATIO, h)
}

pub fn get_piece_pos(_model: &Model, (x, y): (f32, f32)) -> (f32, f32) {
    // _think_ * 8 because board is 800 * 800, vs 100 * 100 viewbox
    (x * 8. - 29.5, y * 8. - 25.5)
}

pub fn piece_to_node(model: &Model, piece: &Piece, pos: (f32, f32)) -> Node<crate::Msg> {
    let (x, y) = get_piece_pos(model, pos);
    let (w, h) = get_piece_dim(model);
    let (x, y) = (x + model.drag.0 * 8., y + model.drag.1 * 8.);


    div![
        style! {
        St::Position => "relative"
        },
        custom![
            Tag::from("piece"),
            C!(piece_class(piece)),
            style! {
                St::Transform => format!("translate({x}px, {y}px)"),
                St::Width => format!("{w}px"),
                St::Height => format!("{h}px"),
            }
        ]
    ]
}

#[inline]
pub fn piece_class(piece: &Piece) -> String {
    use BoardPiece::*;
    use Color::*;
    format!(
        "{piece} {color}",
        piece = match piece.r#type {
            Ant => "ant",
            Queen => "bee",
            Grasshopper => "grasshopper",
            Spider => "spider",
            Beetle => "beetle",
        },
        color = match piece.color {
            White => "white",
            Black => "black",
        }
    )
}

pub fn view_board(model: &Model) -> Node<crate::Msg> {
    svg![
        el_ref(&model.svg),
        ev(Ev::MouseMove, |event| {
            crate::Msg::Game(Msg::Move(event))
        }),
        ev(Ev::MouseUp, |event| {
            event.prevent_default();
            crate::Msg::Game(Msg::Release(event))
        }),
        ev(Ev::MouseDown, |event| {
            crate::Msg::Game(Msg::Click(event))
        }),
        attrs! {
            At::ViewBox => "0 0 100 100",
            At::Draggable => "true",
        },
        defs![g![
            attrs! { At::Id => "pod" },
            polygon![attrs! {
                At::StrokeWidth => ".5",
                At::Points => &model.size,
            },]
        ]],
        model.gridv3.iter().map(|hex| hex.node(model.drag))
    ]
}

pub fn view_pieces(model: &Model) -> Node<crate::Msg> {
    div![model.gridv3.iter().filter_map(|hex| {
        hex.pieces
            .last()
            .map(|p| piece_to_node(model, p, hex.to_pixels()))
    })]
}

#[derive(Clone, Copy)]
pub enum Key {
    Left,
    Right,
}

use std::str::FromStr;
impl FromStr for Key {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ArrowLeft" => Ok(Self::Left),
            "ArrowRight" => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

fn add_yellow(grid: &mut Vec<Hex>, r#move: InternalMove) {
    for hex in grid {
        if hex.sq() == r#move.sq {
            hex.yellow = true;
        }
        if let Some(old_sq) = r#move.old_sq {
            if hex.sq() == old_sq {
                hex.yellow = true;
            }
        }
    }
}

pub fn clear_yellow(grid: &mut Vec<Hex>) {
    for hex in grid {
        hex.yellow = false;
    }
}

pub fn replay_move(model: &mut Model, key: Key) {
    if model.replay_board.is_none() {
        model.replay_board = ReplayBoard::new(model);
    }

    if let Some(replay) = model.replay_board.as_mut() {
        if let Some(index) = replay.get_and_update_index(key) {
            let m = model.game.as_ref().unwrap().move_list[index].clone();

            clear_yellow(&mut model.gridv3);
            match key {
                Key::Left => {
                    if index > 0 {
                        let prev = model.game.as_ref().unwrap().move_list[index - 1].clone();
                        add_yellow(&mut model.gridv3, prev);
                    }

                    replay.board.unplay_move(m);
                }
                Key::Right => {
                    add_yellow(&mut model.gridv3, m.clone());
                    replay.board.play_move_(m);
                }
            };

            grid_from_board_(&mut model.gridv3, &replay.board);
        }
    }
}

pub fn clear_replay(model: &mut Model) {
    if model.replay_board.is_some() {
        model.replay_board = None;
        grid_from_board(model);
    }
}

pub fn remove_top_piece(model: &mut Model, sq: Square) {
    if let Some(hex) = model.gridv3.iter_mut().find(|hex| hex.sq() == sq) {
        hex.remove_top();
    }
}
