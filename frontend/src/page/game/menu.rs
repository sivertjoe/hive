use super::util::*;
use super::Msg;
use seed::{self, prelude::*, *};
use shared::model::game::*;

pub struct MenuItem {
    x: f32,
    y: f32,
    piece: Piece,
}

impl MenuItem {
    pub fn to_menu_node(&self) -> Node<crate::Msg> {
        let (x, y) = (self.x, self.y);

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
                // This is a big yikes; however, good enough for now.

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
            },
            attrs! {
                    At::X => x,
                    At::Y => y,
                    At::Draggable => "true",
            }
        ]
    }
}

pub fn create_menu(color: Color) -> Vec<MenuItem> {
    let deltas = |n: f32| (15. * n, 9. * n);
    let (_, dy) = deltas(0.5);

    use BoardPiece::*;
    let colors = [Queen, Ant, Spider, Grasshopper, Beetle];
    let _spacing = 100.0 / colors.len() as f32;


    // Should be set to the players colors
    let len = colors.len();

    colors
        .into_iter()
        .enumerate()
        .map(|(i, bp)| {
            let piece = Piece { r#type: bp, color };
            let size = 100.0 / len as f32;
            let per = size / 2.0; // center
            let x = ((i + 1) as f32 * size) - per;

            let y = 1.65 * dy; // ???

            MenuItem { x, y, piece }
        })
        .collect()
}
