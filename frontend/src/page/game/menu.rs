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
                St::Display => "block"
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
        let id = format!("{:?}", self.piece.r#type);

        let piece_clone = self.piece;


        let h = 80.;
        // 🧙🧙🧙
        let w = h * (RATIO * 0.99);


        let piece = || -> Node<crate::Msg> {
            custom![
                Tag::from("piece"),
                C!(piece_class(&piece_clone)),
                style! {
                    St::Width => format!("{w}px"),
                    St::Height=> format!("{h}px"),
                }
            ]
        };


        div![
            ev(Ev::DragStart, move |event| {
                let ev = to_drag_event(&event);
                use web_sys::{Element, HtmlDivElement};

                log("dragstart");

                let idv = event
                    .current_target()
                    .unwrap()
                    .dyn_ref::<HtmlDivElement>()
                    .unwrap()
                    .clone();

                let el: &Element = idv.as_ref();
                let pi: Element = el
                    .children()
                    .get_with_index(0)
                    .unwrap()
                    .children()
                    .get_with_index(0)
                    .unwrap();


                let id = el.id();

                ev.data_transfer()
                    .unwrap()
                    .set_data("text/plain", &id)
                    .unwrap();

                ev.data_transfer()
                    .unwrap()
                    .set_drag_image(&pi, (60. * RATIO) as i32, 60);


                crate::Msg::Game(Msg::Drag(piece_clone))
            }),
            id!(&id),
            attrs! {
                At::Draggable => "true",
            },
            style! {
                St::Width => format!("{w}px"),
                St::Height => format!("{h}px"),
                St::Color => "red",
                St::Margin => "5px",
            },
            div![
                style! {
                    St::Position => "relative"
                },
                piece(),
                h1!(self.count_left),
            ],
        ]
    }
}
