use seed::{self, prelude::*, *};
use shared::model::{Game, ResponseBody};
use shared::ObjectId;

pub enum Msg {
    FetchGame(fetch::Result<String>),
    ClickHex(usize),
}

#[derive(Default)]
pub struct Model {
    game: Option<Game>,
    grid: Vec<Node<crate::Msg>>,
    size: String,
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
                    grid: create_grid(2),
                    size,
                })
            }
            _ => None,
        },
        _ => None,
    }
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FetchGame(Ok(text)) => match serde_json::from_str::<ResponseBody>(&text) {
            Ok(resp) => match resp.status {
                200 => {
                    let game: Game = resp.get_body();
                    model.game = Some(game);
                }

                e => {
                    // handle
                }
            },
            Err(e) => {
                // handle
            }
        },

        Msg::FetchGame(Err(text)) => {
            // handle
        }
        Msg::ClickHex(idx) => {
            let node = &mut model.grid[idx];
            if let Node::Element(ref mut el) = node {
                let at = &mut el.attrs;
                at.add(At::Fill, "green");
            } else {
            }
        }
    }
}

pub fn view(model: &Model) -> Node<crate::Msg> {
    div![C!("container"), grid(model),]
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

/* For a number `n`, the number of hexagon is
 * #hexagons = (1 + 6 * (n - 1)!)
 *
 *
 * The height of the thing is:
 * hexagon-height = 2n - 1
 */

fn create_grid(n: usize) -> Vec<Node<crate::Msg>> {
    let deltas = |n: f32| (15. * n, 9. * n);

    let (dx, dy) = deltas(0.5);

    let mut id = 0;
    (0..=n)
        .map(|n| draw_circle(n, dx, dy, &mut id))
        .flatten()
        .collect()
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

fn draw_circle(n: usize, dx: f32, dy: f32, id: &mut usize) -> Vec<Node<crate::Msg>> {
    let (cx, cy) = (50., 50.);
    if n == 0 {
        let res = vec![hex(cy, cy, *id)];
        *id += 1;
        res
    } else {
        let mut sy = cy - (n as f32 * dy * 2.);
        let mut sx = cx;
        HexIter::new(n)
            .into_iter()
            .map(|(zx, zy)| {
                let r = hex(sx, sy, *id);
                sx += 2. * dx * zx;
                sy += 2. * dy * zy;
                *id += 1;
                r
            })
            .collect::<Vec<_>>()
    }
}

fn hex(x: f32, y: f32, id: usize) -> Node<crate::Msg> {
    r#use![
        attrs! {
            At::Href => "#pod",
            At::Transform => format!("translate({x}, {y})"),
            At::Fill => "transparent",
        },
        ev(Ev::Click, move |event| {
            event.prevent_default();
            crate::Msg::Game(Msg::ClickHex(id))
        })
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
                At::Stroke => "gold",
                At::StrokeWidth => ".5",
                At::Points => &model.size,
            },]
        ]],
        g![
            attrs! {
                At::Class => "pod-wrap"
            },
            &model.grid
        ]
    ]
}
