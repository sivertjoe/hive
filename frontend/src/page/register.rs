use seed::{self, prelude::*};
use shared::model::http::*;

use crate::component::user_cred;

pub fn init(_url: Url) -> Option<Model>
{
    Some(Model {
        user_cred: user_cred::init("Register".to_string(), ("register".to_string(), 201)),
    })
}

pub struct Model
{
    pub user_cred: user_cred::Model,
}

pub fn view<Ms: 'static>(model: &Model) -> Node<Ms>
{
    user_cred::view(&model.user_cred)
}
