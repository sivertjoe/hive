use seed::{self, prelude::*, *};
use shared::model::game::*;

struct Orientation {
    f0: f32,
    f1: f32,
    f2: f32,
    f3: f32,

    _b0: f32,
    _b1: f32,
    _b2: f32,
    _b3: f32,

    _start_angle: f32,
}

#[derive(Clone)]
pub struct Hex {
    pub q: isize,
    pub r: isize,
    pub s: isize,

    pub pieces: Vec<Piece>,
    pub selected: bool,

    pub highlight: bool,
    pub red: bool,
}

impl Orientation {
    fn flat() -> Self {
        Self {
            f0: 3.0 / 2.0,
            f1: 0.0,
            f2: 3.0_f32.sqrt() / 2.0,
            f3: 3.0_f32.sqrt(),

            _b0: 2.0 / 3.0,
            _b1: 0.0,
            _b2: -1.0 / 3.0,
            _b3: 3.0_f32.sqrt() / 3.0,
            _start_angle: 0.0,
        }
    }
}

impl Hex {
    pub fn new(q: isize, r: isize, s: isize) -> Self {
        Self {
            q,
            r,
            s,
            pieces: Vec::new(),
            selected: false,
            highlight: false,
            red: false,
        }
    }

    pub fn top(&self) -> Option<&Piece> {
        self.pieces.last()
    }

    pub fn place_piece(&mut self, piece: Piece) {
        self.pieces.push(piece);
    }

    pub fn remove_top(&mut self) -> Option<Piece> {
        self.pieces.pop()
    }

    #[allow(non_snake_case)]
    pub fn to_pixels(&self) -> (f32, f32) {
        let M = Orientation::flat();

        let x: f32 = (M.f0 * self.q as f32 + M.f1 * self.r as f32) * S;
        let y: f32 = (M.f2 * self.q as f32 + M.f3 * self.r as f32) * S;

        (x + 50.0, y + 50.0)
    }

    pub fn sq(&self) -> Square {
        (self.q, self.r, self.s)
    }


    pub fn node(&self) -> Node<crate::Msg> {
        let (x, y) = self.to_pixels();


        let (fill, opacity) = match (self.red, self.selected) {
            (true, _) => ("red", "0.5"),
            (_, true) => ("grey", "0.7"),
            _ => ("transparent", "1.0"),
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

pub fn create_gridv3(r: usize) -> Vec<Hex> {
    use std::cmp::{max, min};
    let r = r as isize;

    let mut vec = Vec::new();
    for q in -r..=r {
        let r1 = max(-r, -q - r);
        let r2 = min(r, -q + r);

        for r in r1..=r2 {
            vec.push(Hex::new(q, r, -q - r));
        }
    }
    vec
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

const _S: f32 = 5.1;
const S: f32 = _S * 0.8;

#[allow(non_snake_case)]
pub fn pixel_to_hex(x: isize, y: isize) -> Square {
    let (x, y) = (x as f32 - 50., y as f32 - 50.);

    let x = x / S;
    let y = y / S;


    let q = 2.0 / 3.0 * x;
    let r = (-1.0 / 3.0) * x + (3.0_f32.sqrt() / 3.0) * y;

    let s = -q - r;

    round(q, r, s)
}
