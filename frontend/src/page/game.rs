use seed::{self, prelude::*, *};

pub fn init(_url: Url) -> Option<Model> {
    Some(Model {})
}

pub struct Model {}

/* For a number `n`, the number of hexagon is
 * #hexagons = (1 + 6 * (n - 1)!)
 *
 *
 * The height of the thing is:
 * hexagon-height = 2n - 1
 */

pub fn grid<Ms: 'static>(model: &Model) -> Node<Ms> {
    let off = 60 / 2;

    let n = 8;
    let min = 4;
    let pat = (min..=n).chain((min..n).rev()).collect::<Vec<_>>();
    let max = pat.iter().max().unwrap();

    div![(0..pat.len()).map(|i| {
        let offset = off * (max - pat[i]);
        div![
            C!("hex-row"),
            style! {
                St::MarginLeft => format!("{offset}px")
            },
            (0..pat[i]).map(|_| { div![C!("hexagon"),] })
        ]
    })]
}

pub fn view<Ms: 'static>(model: &Model) -> Node<Ms> {
    div![C!("container"), grid(model),]
}
