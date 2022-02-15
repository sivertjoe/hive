use seed::{prelude::*, *};

use crate::{Model, Msg};

pub fn navbar(model: &Model) -> Node<Msg>
{
    div![C!("navbar"), div![C!("menu"), div![C!("items"), h2!["register",]]]]
}
