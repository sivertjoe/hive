use seed::{prelude::*, *};

pub struct Model {
    pub items: Vec<(String, bool)>,
}

pub fn init(items: Vec<String>) -> Model {
    Model {
        items: items.into_iter().map(|item| (item, true)).collect(),
    }
}
pub fn view<Ms: 'static>(model: &Model) -> Node<Ms> {
    div![select![model.items.iter().filter_map(|(label, flag)| {
        flag.then(|| Some(option![attrs! {At::Value => label }]))
    })]]
}
