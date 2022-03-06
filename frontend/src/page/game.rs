use seed::{self, prelude::*, *};
use shared::model::{Game, ResponseBody};
use shared::{model::game::*, ObjectId};

pub enum Msg {
    FetchGame(fetch::Result<String>),
    ClickHex(usize),

    SelectPiece(usize),
}

struct Hex {
    x: f32,
    y: f32,
    piece: Option<Piece>,
    idx: usize,
    selected: bool,
}

impl Hex {
    fn empty(x: f32, y: f32, idx: usize) -> Self {
        Self {
            x,
            y,
            idx,
            piece: None,
            selected: false,
        }
    }

    fn node(&self) -> Node<crate::Msg> {
        let x = self.x;
        let y = self.y;
        let idx = self.idx;

        let opacity = match self.selected {
            true => "0.5",
            false => "1.0",
        };

        let fill = match (self.piece.as_ref(), self.selected) {
            (Some(p), _) => piece_color(p.r#type),
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

#[derive(Default)]
pub struct Model {
    game: Option<Game>,

    gridv2: Vec<Hex>,
    menu: Vec<(Node<crate::Msg>, BoardPiece)>,

    selected_piece: Option<(Piece, Option<usize>)>,

    size: String,
    label: Option<String>,
}

/* NOTE
 * we must use `gen_size` number in create_grid!!!
 */

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
                    //grid: create_grid(2),
                    gridv2: create_gridv2(2),
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
            let hex = &mut model.gridv2[idx];
            if let Some((p, _)) = &model.selected_piece {
                hex.piece = Some(*p);
                clear_squares(model);
                let (piece, _) = model.selected_piece.take().unwrap();
                move_piece(model, piece, idx);
            }
        }
        Msg::SelectPiece(idx) => {
            update_menu(model, idx);
            update_squares(model);
        }
    }
}

fn move_piece(model: &mut Model, p: Piece, idx: usize) {
    let board = &mut model.game.as_mut().unwrap().board;
    board.place_piece(p, idx);

    // send the request maybe
}

fn clear_squares(model: &mut Model) {
    for square in &mut model.gridv2 {
        square.selected = false;
    }
}

fn update_squares(model: &mut Model) {
    let board = &model.game.as_ref().unwrap().board;
    let (piece, pos) = &model.selected_piece.as_ref().unwrap();

    for mov in legal_moves(piece, board, pos) {
        model.gridv2[mov as usize].selected = true;
    }
}

fn update_menu(model: &mut Model, idx: usize) {
    for (i, (item, bp)) in model.menu.iter_mut().enumerate() {
        if let Node::Element(ref mut el) = item {
            let at = &mut el.attrs.vals;
            if i == idx {
                let p = Piece {
                    color: Color::White,
                    r#type: *bp,
                };
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

fn create_gridv2(n: usize) -> Vec<Hex> {
    let deltas = |n: f32| (15. * n, 9. * n);

    let (dx, dy) = deltas(0.5);

    let mut id = 0;
    (0..=n)
        .map(|n| draw_circlev2(n, dx, dy, &mut id))
        .flatten()
        .collect()
}

fn draw_circlev2(n: usize, dx: f32, dy: f32, id: &mut usize) -> Vec<Hex> {
    let (cx, cy) = (50., 50.);
    if n == 0 {
        let res = vec![Hex::empty(cx, cy, *id)];
        *id += 1;
        res
    } else {
        let mut sy = cy - (n as f32 * dy * 2.);
        let mut sx = cx;
        HexIter::new(n)
            .into_iter()
            .map(|(zx, zy)| {
                let res = Hex::empty(sx, sy, *id);
                sx += 2. * dx * zx;
                sy += 2. * dy * zy;
                *id += 1;
                res
            })
            .collect::<Vec<_>>()
    }
}

struct HexIter {
    n: usize,
    it: usize,
}

impl HexIter {
    fn new(n: usize) -> Self {
        HexIter { n, it: 0 }
    }
}

impl Iterator for HexIter {
    type Item = (f32, f32);

    fn next(&mut self) -> Option<Self::Item> {
        const SEQ: [(f32, f32); 6] = [
            (0.5, 0.5),
            (0.0, 1.0),
            (-0.5, 0.5),
            (-0.5, -0.5),
            (0.0, -1.0),
            (0.5, -0.5),
        ];

        if self.it < 6 * self.n {
            let idx = self.it / self.n;
            self.it += 1;
            Some(SEQ[idx])
        } else {
            None
        }
    }
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

fn piece_color(b: BoardPiece) -> &'static str {
    match b {
        BoardPiece::Queen => "gold",
        BoardPiece::Ant => "blue",
        BoardPiece::Spider => "brown",
        BoardPiece::Grasshopper => "green",
        BoardPiece::Beetle => "purple",
    }
}

fn create_menu() -> Vec<(Node<crate::Msg>, BoardPiece)> {
    let deltas = |n: f32| (15. * n, 9. * n);
    let (dx, dy) = deltas(0.5);

    use BoardPiece::*;
    let colors = [Queen, Ant, Spider, Grasshopper, Beetle];
    let spacing = 100.0 / colors.len() as f32;

    colors
        .into_iter()
        .enumerate()
        .map(|(i, piece)| {
            (
                piece_hex(dx, dy, colors.len(), i, i, piece_color(piece), spacing),
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
        model.gridv2.iter().map(|hex| hex.node())
    ]
}
