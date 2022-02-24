use seed::{prelude::*, *};

use crate::{Model, Msg};


fn capitalize(s: &str) -> String
{
    let mut c = s.chars();
    match c.next()
    {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn create_link(text: &str) -> Node<Msg>
{
    let _text = capitalize(text);

    h2![a![_text, attrs! { At::Href => text }]]
}

pub fn view(model: &Model) -> Node<Msg>
{
    div![C!("navbar"), div![C!("menu"), div![
        C!("items"),
        h2![a!["Home", attrs! { At::Href => "/" }]], // Does not go to the page??
        create_link("register"),
        create_link("login"),
        create_link("create"),
        IF!(model.user.is_some() => match &model.user {
            Some(name) => create_link(&name),
            _ => unreachable!()
        })
    ]]]
}
