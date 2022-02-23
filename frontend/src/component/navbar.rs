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

pub fn view(_model: &Model) -> Node<Msg>
{
    div![C!("navbar"), div![C!("menu"), div![
        C!("items"),
        create_link("register"),
        create_link("login")
    ]]]
}
