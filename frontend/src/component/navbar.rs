use seed::{prelude::*, *};

use crate::{Model, Msg, Urls};

fn create_link(text: &str, url: Url) -> Node<Msg> {
    h2![a![text, attrs! { At::Href => url }]]
}

pub fn view(model: &Model) -> Node<Msg> {
    let url = || Urls::new(&model.base_url);
    div![
        C!("navbar"),
        div![
            C!("menu"),
            div![
                C!("items"),
                create_link("Home", url().home()),
                create_link("Register", url().register()),
                create_link("Login", url().login()),
                create_link("Create", url().create()),
                IF!(model.user.is_some() => match &model.user {
                    Some(name) => create_link(name, url().user(name)),
                    _ => unreachable!()
                })
            ]
        ]
    ]
}
