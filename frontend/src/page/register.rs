use seed::{self, prelude::*};

use crate::component::user_cred;

pub fn init(_url: Url) -> Option<Model> {
    Some(Model {
        user_cred: user_cred::init("Register".to_string(), ("register".to_string(), 201)),
    })
}

pub enum Msg {
    Component(user_cred::Msg),
}

pub struct Model {
    pub user_cred: user_cred::Model,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<crate::Msg>) {
    match msg {
        Msg::Component(msg) => {
            let f = |msg| crate::Msg::RegisterPage(Msg::Component(msg));
            user_cred::update(msg, &mut model.user_cred, orders, f)
        }
    }
}

pub fn view(model: &Model) -> Node<crate::Msg> {
    let f = |msg| crate::Msg::RegisterPage(Msg::Component(msg));
    user_cred::view(&model.user_cred, f)
}
