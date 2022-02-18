use seed::{self, prelude::*, *};
use serde::Serialize;
use shared::model::UserCredentials;

use crate::Msg::Register;

pub fn init(mut url: Url) -> Option<Model>
{
    Some(Model {
        form: UserCredentials::default(), text: None
    })
}


pub struct Model
{
    form: UserCredentials,
    text: Option<Status>,
}


enum RemoteData<T>
{
    NotAsked,
    Loading,
    Loaded(T),
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
    Fetched(fetch::Result<String>),
}

async fn send_register_message(form: UserCredentials) -> fetch::Result<String>
{
    Request::new("http://0.0.0.0:5000/register")
        .method(Method::Post)
        .json(&form)?
        .fetch()
        .await?
        .check_status()?
        //.json()
        .text()
        .await
}


pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>)
{
    match msg
    {
        Msg::Submit =>
        {
            let form = model.form.clone();
            orders
                .skip()
                .perform_cmd(async { Msg::Fetched(send_register_message(form).await) });
        },
        Msg::NameChanged(name) =>
        {
            model.form.name = name;
        },
        Msg::PasswordChanged(password) =>
        {
            model.form.password = password;
        },

        Msg::Fetched(Ok(text)) =>
        {
            model.text = Some(Status::Success(text));
        },

        Msg::Fetched(Err(text)) =>
        {
            model.text = Some(Status::Error(format!("{text:?}")));
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
                attrs! { At::Value => model.form.name, At::Placeholder => "name" },
                input_ev(Ev::Input, |name| { Register(Msg::NameChanged(name)) }),
            ],
            br!(),
            br!(),
            input![
                attrs! { At::Value => model.form.password, At::Type => "password", At::Placeholder => "password" },
                input_ev(Ev::Input, |password| { Register(Msg::PasswordChanged(password)) }),
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
