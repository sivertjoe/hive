use seed::{self, prelude::*};

use crate::component::user_cred;

pub fn init(_url: Url) -> Option<Model> {
    Some(Model {
        user_cred: user_cred::init("Login".to_string(), ("login".to_string(), 200)),
    })
}

pub fn update(model: &mut Model) {}

pub struct Model {
    pub user_cred: user_cred::Model,
}

pub fn view<Ms: 'static>(model: &Model) -> Node<Ms> {
    user_cred::view(&model.user_cred)
}
