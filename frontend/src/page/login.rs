use seed::{prelude::*, *};

pub fn init(mut url: Url) -> Option<Model>
{
    Some(Model {
        username: String::new(), password: String::new()
    })
}


pub struct Model
{
    username: String,
    password: String,
}

// @NOTE: Generics??
pub fn view<Ms>() -> Node<Ms>
{
    h1!["login"]
}
