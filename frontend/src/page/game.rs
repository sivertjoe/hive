use seed::{self, prelude::*, *};
use shared::model::{Game, ResponseBody};
use shared::{model::game::*, ObjectId};
use web_sys::SvgGraphicsElement;
use web_sys::{Event, MouseEvent};

#[derive(Default)]
pub struct Model {
    game: Option<Game>,

    gridv3: Vec<_Hex>,

    menu: Vec<(Node<crate::Msg>, BoardPiece)>,

    piece: Option<_Hex>,

    svg: ElRef<SvgGraphicsElement>,

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
                //orders.perform_cmd(async move { Msg::FetchGame(send_message(id).await) });
                let size = gen_size(0.5);


                let arr = [
                    ObjectId::parse_str("507f1f77bcf86cd799439011").unwrap(),
                    ObjectId::parse_str("507f1f77bcf86cd799439011").unwrap(),
                ];

                let mut gridv3 = create_gridv3(1);
                gridv3
                    .iter_mut()
                    .find(|h| h.sq() == (0, 0, 0))
                    .unwrap()
                    .piece = Some(Piece {
                    r#type: BoardPiece::Ant,
                    color: Color::White,
                });

                Some(Model {
                    game: Some(Game::new(arr)),
                    //gridv3: create_gridv3(5),
                    gridv3,

                    svg: ElRef::default(),

                    menu: create_menu(),
                    piece: None,
                    size,
                    label: None,
                })
            }
            _ => None,
        },
        _ => None,
    }
}

pub enum Msg {
    FetchGame(fetch::Result<String>),
    Move(Event),
    Click(Event),
    Release(Event),
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

        Msg::Click(event) => {
            let mm = to_mouse_event(&event);

            let (x, y) = get_mouse_pos(model, mm);
            let sq = pixel_to_hex(x as isize, y as isize);


            model.piece = model
                .gridv3
                .iter()
                .find(|hex| hex.sq() == sq && hex.piece.is_some())
                .map(|hex| {
                    let mut hex = hex.clone();
                    hex._x = x;
                    hex._y = y;
                    hex
                })
        }

        Msg::Release(event) => {
            let mm = to_mouse_event(&event);
            let (x, y) = get_mouse_pos(model, mm);
            let sq = pixel_to_hex(x as isize, y as isize);

            if let Some(sel_hex) = model.piece.take() {
                if let Some(hex) = model.gridv3.iter_mut().find(|hex| hex.sq() == sq) {
                    hex.piece = sel_hex.piece;
                }

                if let Some(old_hex) = model.gridv3.iter_mut().find(|hex| hex.sq() == sel_hex.sq())
                {
                    old_hex.piece = None;
                }
            } else {
                // place back or something
            }
        }

        Msg::Move(event) => {
            let mm = to_mouse_event(&event);

            let (x, y) = get_mouse_pos(model, mm);
            if let Some(piece) = model.piece.as_mut() {
                piece._x = x;
                piece._y = y;
            }
        }
    }
}

fn get_mouse_pos(model: &Model, mm: &MouseEvent) -> (f32, f32) {
    let ctm = model.svg.get().unwrap().get_screen_ctm().unwrap();
    let (x, y) = (mm.client_x(), mm.client_y());
    let (x, y) = (x as f32, y as f32);

    ((x - ctm.e()) / ctm.a(), (y - ctm.f()) / ctm.d())
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
    r#use![attrs! {
        At::Href => "#pod",
        At::Transform => format!("translate({x}, {y})"),
        At::Display => "flex",
        At::Fill => stroke,
        //At::Stroke => stroke,
        At::Width => spacing,
        //At::Class => "selected-piece",
    },]
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
        el_ref(&model.svg),
        ev(Ev::MouseMove, |event| {
            crate::Msg::Game(Msg::Move(event))
        }),
        ev(Ev::MouseUp, |event| {
            log("test??");
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
                //At::Stroke => "gold",
                At::StrokeWidth => ".5",
                At::Points => &model.size,
            },]
        ]],
        model.gridv3.iter().map(_Hex::_node),
        IF!(model.piece.is_some() => {
            let p = model.piece.as_ref().unwrap();
            p._node_xy()
        })
    ]
}

/// HEX STUFF
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

fn round(_q: f32, _r: f32, _s: f32) -> Square {
    let mut q = _q.round();
    let mut r = _r.round();
    let mut s = _s.round();

    let q_diff = (q - _q).abs();
    let r_diff = (r - _r).abs();
    let s_diff = (s - _s).abs();

    if q_diff > r_diff && q_diff > s_diff {
        q = -r - s;
    } else if r_diff > s_diff {
        r = -q - s;
    } else {
        s = -q - r;
    }
    (q as isize, r as isize, s as isize)
}

#[allow(non_snake_case)]
pub fn pixel_to_hex(x: isize, y: isize) -> Square {
    let (x, y) = (x as f32 - 50., y as f32 - 50.);
    const S: f32 = 5.1;

    let x = x / S;
    let y = y / S;


    let q = 2.0 / 3.0 * x;
    let r = (-1.0 / 3.0) * x + (3.0_f32.sqrt() / 3.0) * y;

    let s = -q - r;

    let f = round(q, r, s);
    //(f.0 - 49, f.1 - 24, f.2 + 73)
    f
}

#[derive(Clone)]
pub struct _Hex {
    q: isize,
    r: isize,
    s: isize,

    _x: f32,
    _y: f32,

    idx: usize,
    piece: Option<Piece>,
    selected: bool,

    highlight: bool,
}

impl _Hex {
    #[allow(non_snake_case)]
    fn to_pixels(&self) -> (f32, f32) {
        let M = Orientation::flat();
        const S: f32 = 5.1;

        let x: f32 = (M.f0 * self.q as f32 + M.f1 * self.r as f32) * S;
        let y: f32 = (M.f2 * self.q as f32 + M.f3 * self.r as f32) * S;

        (x + 50.0, y + 50.0)
    }

    fn sq(&self) -> Square {
        (self.q, self.r, self.s)
    }

    fn _node_xy(&self) -> Node<crate::Msg> {
        let (x, y) = (self._x, self._y);
        let opacity = match self.selected {
            true => "0.5",
            false => "1.0",
        };

        let fill = match self.piece.as_ref() {
            Some(p) => piece_color(p.r#type, p.color),
            None => "transparent",
        };

        r#use![attrs! {
            At::Href => "#pod",
            At::Transform => format!("translate({x}, {y})"),
            At::Fill => fill,
            At::Stroke => "gold",
            At::Opacity => opacity,
        },]
    }

    fn _node(&self) -> Node<crate::Msg> {
        let (x, y) = self.to_pixels();

        let opacity = match self.selected {
            true => "0.5",
            false => "1.0",
        };

        let fill = match (self.piece.as_ref(), self.selected) {
            (Some(p), _) => piece_color(p.r#type, p.color),
            (None, false) => "transparent",
            (None, true) => "grey",
        };

        let c = if self.highlight { "selected-piece" } else { "" };

        r#use![attrs! {
            At::Href => "#pod",
            At::Transform => format!("translate({x}, {y})"),
            At::Fill => fill,
            At::Stroke => "gold",
            At::Opacity => opacity,
            At::Class => c,
            At::DropZone => "move",
        },]
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
                highlight: false,
                _x: 0.0,
                _y: 0.0,
            });
            idx += 1;
        }
    }
    vec
}
