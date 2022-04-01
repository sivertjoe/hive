use super::util::*;
use super::Msg;
use seed::{self, prelude::*, *};
use shared::model::*;

pub struct Menu {
    items: Vec<MenuEntry>,
}

impl Menu {
    pub fn new<I: Iterator<Item = Piece>>(pieces: I, board: &Board) -> Self {
        let items = pieces
            .map(|piece| MenuEntry {
                count_left: piece_to_count(piece.r#type),
                piece,
            })
            .collect();

        let mut menu = Menu { items };
        let color = menu.items.first().unwrap().piece.color;

        for square in board.values() {
            for bp in square
                .pieces
                .iter()
                .filter_map(|piece| (piece.color == color).then(|| piece.r#type))
            {
                menu.reduce_piece(bp);
            }
        }


        menu
    }
    pub fn to_node(&self) -> Node<crate::Msg> {
        div![
            style! {
                St::Display => "flex",
            },
            self.items
                .iter()
                .filter_map(|entry| (entry.count_left > 0).then(|| entry.to_node()))
        ]
    }

    pub fn reduce_piece(&mut self, bp: BoardPiece) {
        self.items
            .iter_mut()
            .find(|entry| entry.piece.r#type == bp)
            .unwrap()
            .count_left -= 1;
    }
}

#[inline]
fn piece_to_count(bp: BoardPiece) -> usize {
    match bp {
        BoardPiece::Ant => 3,
        BoardPiece::Grasshopper => 3,
        BoardPiece::Spider => 2,
        BoardPiece::Beetle => 2,
        BoardPiece::Queen => 1,
    }
}

// Maybe rename stuff later
pub struct MenuEntry {
    count_left: usize,
    piece: Piece,
}

impl MenuEntry {
    pub fn to_node(&self) -> Node<crate::Msg> {
        let stroke = piece_color(self.piece.r#type, self.piece.color);
        let id = format!("{:?}", self.piece.r#type);

        let piece_clone = self.piece;
        div![
            ev(Ev::DragStart, move |event| {
                let ev = to_drag_event(&event);
                use web_sys::{Element, HtmlDivElement};

                let idv = event
                    .current_target()
                    .unwrap()
                    .dyn_ref::<HtmlDivElement>()
                    .unwrap()
                    .clone();

                let el: &Element = idv.as_ref();
                let id = el.id();

                ev.data_transfer()
                    .unwrap()
                    .set_data("text/plain", &id)
                    .unwrap();


                crate::Msg::Game(Msg::Drag(piece_clone))
            }),
            id!(&id),
            style! {
                    St::Width => "90px",//self.spacing,
                    St::Height => "90px", // ??
                    St::Background => stroke,
                    St::Color => "black",
            },
            attrs! {
                    At::Draggable => "true",
            },
            h1!(self.count_left)
        ]
    }
}
