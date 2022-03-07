use seed::{self, prelude::*, *};
use shared::model::{Game, ResponseBody};
use shared::{model::game::*, ObjectId};

pub enum Msg {
    FetchGame(fetch::Result<String>),
    ClickHex(usize),

    SelectPiece(usize),
}

struct Orientation {
    f0: f32,
    f1: f32,
    f2: f32,
    f3: f32,

    b0: f32,
    b1: f32,
    b2: f32,
    b3: f32,

    start_angle: f32,
}

impl Orientation {
    fn flat() -> Self {
        Self {
            f0: 3.0 / 2.0,
            f1: 0.0,
            f2: 3.0_f32.sqrt() / 2.0,
            f3: 3.0_f32.sqrt(),

            b0: 2.0 / 3.0,
            b1: 0.0,
            b2: -1.0 / 3.0,
            b3: 3.0_f32.sqrt() / 3.0,

            start_angle: 0.0,
        }
    }
}

struct _Hex {
    q: isize,
    r: isize,
    s: isize,

    idx: usize,
    piece: Option<Piece>,
    selected: bool,
}

impl _Hex {
    fn sq(&self) -> Square {
        (self.q, self.r, self.s)
    }

    #[allow(non_snake_case)]
    fn to_pixels(&self) -> (f32, f32) {
        let M = Orientation::flat();
        const S: f32 = 5.1;

        let x: f32 = (M.f0 * self.q as f32 + M.f1 * self.r as f32) * S;
        let y: f32 = (M.f2 * self.q as f32 + M.f3 * self.r as f32) * S;

        (x + 50.0, y + 50.0)
    }

    fn _node(&self) -> Node<crate::Msg> {
        let (x, y) = self.to_pixels();
        let idx = self.idx;

        let opacity = match self.selected {
            true => "0.5",
            false => "1.0",
        };

        let fill = match (self.piece.as_ref(), self.selected) {
            (Some(p), _) => piece_color(p.r#type, p.color),
            (None, false) => "transparent",
            (None, true) => "grey",
        };

        r#use![
            attrs! {
                At::Href => "#pod",
                At::Transform => format!("translate({x}, {y})"),
                At::Fill => fill,
                At::Stroke => "gold",
                At::Opacity => opacity,
            },
            ev(Ev::Click, move |event| {
                event.prevent_default();
                crate::Msg::Game(Msg::ClickHex(idx))
            })
        ]
    }
}

fn create_gridv3(r: usize) -> Vec<_Hex> {
    use std::cmp::{max, min};
    let r = r as isize;

    let mut vec = Vec::new();
    let mut idx = 0;
    for q in -r..=r {
        let r1 = max(-r, -q - r);
        let r2 = min(r, -q + r);

        for r in r1..=r2 {
            vec.push(_Hex {
                q,
                r,
                s: -q - r,
                selected: false,
                piece: None,
                idx,
            });
            idx += 1;
        }
    }
    vec
}

#[derive(Default)]
pub struct Model {
    game: Option<Game>,

    gridv3: Vec<_Hex>,

    menu: Vec<(Node<crate::Msg>, BoardPiece)>,

    selected_piece: Option<(Piece, Option<usize>)>,

    size: String,
    label: Option<String>,
}

pub fn init(mut url: Url, orders: &mut impl Orders<Msg>) -> Option<Model> {
    let gen_size = |n: f32| {
        let l = 5. * n;
        let h = 9. * n;
        let w = 10. * n;

        format!("{l}, -{h} -{l}, -{h} -{w}, 0 -{l}, {h} {l}, {h} {w}, 0")
    };
    match url.next_path_part() {
        Some(id) => match ObjectId::parse_str(id) {
            Ok(id) => {
                orders.perform_cmd(async move { Msg::FetchGame(send_message(id).await) });
                let size = gen_size(0.5);
                Some(Model {
                    game: None,
                    gridv3: create_gridv3(2),
                    menu: create_menu(),
                    selected_piece: None,
                    size,
                    label: None,
                })
            }
            _ => None,
        },
        _ => None,
    }
}

pub fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FetchGame(Ok(text)) => match serde_json::from_str::<ResponseBody>(&text) {
            Ok(resp) => match resp.status {
                200 => {
                    let game: Game = resp.get_body();
                    model.game = Some(game);
                }

                e => {
                    model.label = Some(format!("expected 200 got {e}"));
                }
            },
            Err(e) => {
                model.label = Some(format!("serde error: {e}"));
            }
        },

        Msg::FetchGame(Err(text)) => {
            model.label = Some(format!("http error: {text:?}"));
        }
        Msg::ClickHex(idx) => {
            let hex = &mut model.gridv3[idx];
            let sq = hex.sq();

            if let Some((p, _)) = &model.selected_piece {
                hex.piece = Some(*p);
                clear_squares(model);
                let (piece, _) = model.selected_piece.take().unwrap();

                move_piece(model, piece, sq);
            }
        }
        Msg::SelectPiece(idx) => {
            update_menu(model, idx);
            update_squares(model);
        }
    }
}

fn move_piece(model: &mut Model, p: Piece, sq: Square) {
    let board = &mut model.game.as_mut().unwrap().board;
    board.place_piece(p, sq);

    // send the request maybe
}

fn clear_squares(model: &mut Model) {
    for square in &mut model.gridv3 {
        square.selected = false;
    }
}

fn update_squares(model: &mut Model) {
    let board = &model.game.as_ref().unwrap().board;
    let (piece, pos) = &model.selected_piece.as_ref().unwrap();


    let moves = format!("{:?}", legal_moves(piece, board, pos));
    log(moves);

    let b = format!("{:?}", board);
    log(b);


    for mov in legal_moves(piece, board, pos) {
        let hex = model.gridv3.iter_mut().find(|hex| hex.sq() == mov).unwrap();
        hex.selected = true;
    }
}

fn update_menu(model: &mut Model, idx: usize) {
    for (i, (item, bp)) in model.menu.iter_mut().enumerate() {
        if let Node::Element(ref mut el) = item {
            let at = &mut el.attrs.vals;
            if i == idx {
                // TODO: Get correct color
                let color = if model.game.as_ref().unwrap().board.turns % 2 == 0 {
                    Color::White
                } else {
                    Color::Black
                };
                let p = Piece { color, r#type: *bp };
                model.selected_piece = Some((p, None));
                at.insert(At::Class, AtValue::Some("selected-piece".into()));
            } else {
                at.remove(&At::Class);
            }
        }
    }
}

pub fn view(model: &Model) -> Node<crate::Msg> {
    div![div![
        C!("container"),
        grid(model),
        div![C!("piece-menu"), piece_menu(model)],
        IF!(model.label.is_some() => match model.label {
            Some(ref s) => h2! [C!("error"), s],
            _ => unreachable!()
        })
    ]]
}

async fn send_message(id: ObjectId) -> fetch::Result<String> {
    Request::new(format!("http://0.0.0.0:5000/game?q={id}"))
        .method(Method::Get)
        .fetch()
        .await?
        .check_status()?
        .text()
        .await
}

fn piece_hex(
    _dx: f32,
    dy: f32,
    len: usize,
    i: usize,
    id: usize,
    stroke: &str,
    spacing: f32,
) -> Node<crate::Msg> {
    let size = 100.0 / len as f32;
    let per = size / 2.0; // center

    let x = ((i + 1) as f32 * size) - per;


    let y = 1.65 * dy; // ???
    r#use![
        attrs! {
            At::Href => "#pod",
            At::Transform => format!("translate({x}, {y})"),
            At::Display => "flex",
            At::Fill => stroke,
            //At::Stroke => stroke,
            At::Width => spacing,
            //At::Class => "selected-piece",
        },
        ev(Ev::Click, move |event| {
            event.prevent_default();
            crate::Msg::Game(Msg::SelectPiece(id))
        })
    ]
}

fn piece_color(b: BoardPiece, color: Color) -> &'static str {
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

fn create_menu() -> Vec<(Node<crate::Msg>, BoardPiece)> {
    let deltas = |n: f32| (15. * n, 9. * n);
    let (dx, dy) = deltas(0.5);

    use BoardPiece::*;
    let colors = [Queen, Ant, Spider, Grasshopper, Beetle];
    let spacing = 100.0 / colors.len() as f32;


    // Should be set to the players colors
    let color = Color::White;

    colors
        .into_iter()
        .enumerate()
        .map(|(i, piece)| {
            (
                piece_hex(
                    dx,
                    dy,
                    colors.len(),
                    i,
                    i,
                    piece_color(piece, color),
                    spacing,
                ),
                piece,
            )
        })
        .collect()
}

fn piece_menu(model: &Model) -> Node<crate::Msg> {
    svg![
        attrs! {
            At::ViewBox => "0 0 100 15",

        },
        defs![g![
            attrs! { At::Id => "pod" },
            polygon![attrs! {
                At::Stroke => "red",
                At::StrokeWidth => ".5",
                At::Points => &model.size,
            },]
        ]],
        model.menu.iter().map(|p| &p.0)
    ]
}

pub fn grid(model: &Model) -> Node<crate::Msg> {
    svg![
        attrs! {
            At::ViewBox => "0 0 100 100"
        },
        defs![g![
            attrs! { At::Id => "pod" },
            polygon![attrs! {
                //At::Stroke => "gold",
                At::StrokeWidth => ".5",
                At::Points => &model.size,
            },]
        ]],
        model.gridv3.iter().map(_Hex::_node)
    ]
}
