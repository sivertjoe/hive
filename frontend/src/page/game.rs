use seed::{self, prelude::*, *};
use shared::model::{Game, ResponseBody};
use shared::ObjectId;

pub fn init(mut url: Url, orders: &mut impl Orders<Msg>) -> Option<Model> {
    match url.next_path_part() {
        Some(id) => match ObjectId::parse_str(id) {
            Ok(id) => {
                orders.perform_cmd(async move { Msg::FetchGame(send_message(id).await) });
                Some(Model::default())
            }
            _ => None,
        },
        _ => None,
    }
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
    }
}

pub enum Msg {
    FetchGame(fetch::Result<String>),
}

#[derive(Default)]
pub struct Model {
    game: Option<Game>,
}

/* For a number `n`, the number of hexagon is
 * #hexagons = (1 + 6 * (n - 1)!)
 *
 *
 * The height of the thing is:
 * hexagon-height = 2n - 1
 */

fn hex<Ms: 'static>(x: f32, y: f32, id: i32) -> Node<Ms> {
    r#use![
        attrs! {
            At::Href => "#pod",
            At::Transform => format!("translate({x}, {y})"),
        },
        ev(Ev::Click, move |event| {
            log(format!("{id}"));
        })
    ]

    // <use xlink:href="#pod" transform="translate(50, 41)"/>
}

pub fn grid<Ms: 'static>(model: &Model) -> Node<Ms> {
    let rows = 12;
    let cols = 10;

    let gen_size = |n: f32| {
        let l = 5. * n;
        let h = 9. * n;
        let w = 10. * n;

        format!("{l}, -{h} -{l}, -{h} -{w}, 0 -{l}, {h} {l}, {h} {w}, 0")
    };

    let start_points = |n: f32| ((5. + 1.) * n * 2., (9. + 1.) * 2. * n);
    let deltas = |n: f32| (15. * n, 9. * n);

    let (x, y) = start_points(0.5);
    let (dx, dy) = deltas(0.5);
    let size = gen_size(0.5);

    let hexes = (0..rows * cols).map(|i| {
        let x = x + ((i % rows) as f32 * dx);
        let y = ((i / rows) as f32 * 2. * dy) + y + if i & 1 == 0 { -dy } else { 0. };
        //let y = i / cols;*/
        hex(x, y, i)
    });

    svg![
        attrs! {
            At::ViewBox => "0 0 100 100"
        },
        defs![g![
            attrs! { At::Id => "pod" },
            polygon![attrs! {
                At::Stroke => "gold",
                At::StrokeWidth => ".5",
                At::Points => size,
            }]
        ]],
        g![
            attrs! {
                At::Class => "pod-wrap"
            },
            hexes,
        ]
    ]
}

pub fn view<Ms: 'static>(model: &Model) -> Node<Ms> {
    div![C!("container"), grid(model),]
}
