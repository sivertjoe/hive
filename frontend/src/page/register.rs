use seed::{prelude::*, *};

pub fn init(mut url: Url) -> Option<Model>
{
    Some(Model {
        name: String::new(), password: String::new(), text: None
    })
}

pub struct Model
{
    name:     String,
    password: String,
    text:     Option<Status>,
}


enum Status
{
    Success(String),
    Error(String),
}


pub enum Msg
{
    Submit,
    NameChanged(String),
    PasswordChanged(String),
}


pub fn update(msg: Msg, model: &mut Model)
{
    match msg
    {
        Msg::Submit =>
        {
            model.text = Some(Status::Success("Success".to_string()));
        },
        Msg::NameChanged(name) =>
        {
            model.name = name;
        },
        Msg::PasswordChanged(password) =>
        {
            model.password = password;
        },
    }
}


/* @NOTE:
 * Why do I have to write crate::Msg::Register.. instead of must
 * Msg::Submit??
 */
pub fn view<Ms: 'static>(model: &Model) -> Node<Ms>
{
    form![
        C!["container"],
        h1!["Register a new user"],
        ev(Ev::Submit, |event| {
            event.prevent_default();
            crate::Msg::Register(Msg::Submit)
        }),
        div![
            C!("inputs"),
            input![
                C!("inputs"),
                attrs! { At::Value => model.name, At::Placeholder => "name" },
                input_ev(Ev::Input, |name| { crate::Msg::Register(Msg::NameChanged(name)) }),
            ],
            br!(),
            br!(),
            input![
                attrs! { At::Value => model.password, At::Type => "password", At::Placeholder => "password" },
                input_ev(Ev::Input, |password| {
                    crate::Msg::Register(Msg::PasswordChanged(password))
                }),
            ]
        ],
        div![C!("center-button"), button![C!["button"], "Register"]],
        IF!(model.text.is_some() => match model.text {
            Some(Status::Success(ref s)) => h2! [C!("success"), s],
            Some(Status::Error(ref s)) => h2! [C!("error"), s],
            _ => unreachable!()
        })
    ]
}
