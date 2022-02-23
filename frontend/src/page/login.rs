use seed::{self, prelude::*, *};
use shared::model::{http::*, UserCredentials};

use crate::Msg::Login;

pub fn init(_url: Url) -> Option<Model>
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

async fn send_message(form: UserCredentials) -> fetch::Result<String>
{
    Request::new("http://0.0.0.0:5000/login")
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
            orders.skip().perform_cmd(async { Msg::Fetched(send_message(form).await) });
        },
        Msg::NameChanged(name) =>
        {
            model.form.name = name;
        },
        Msg::PasswordChanged(password) =>
        {
            model.form.password = password;
        },

        Msg::Fetched(Ok(text)) => match serde_json::from_str::<ResponseBody>(&text)
        {
            Ok(resp) => match resp.status
            {
                201 =>
                {
                    model.text = Some(Status::Success("Success".into()));
                    let uuid: String = resp.get_body();
                    LocalStorage::insert("uuid", &uuid).expect("inserting uuid in LocalStorage");
                },

                e =>
                {
                    let err: String = resp.get_body();
                    model.text = Some(Status::Error(format!("Error ({e}): {err}")));
                },
            },
            Err(e) =>
            {
                model.text = Some(Status::Error(format!("deserialize error, {e:?}")));
            },
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
        h1!["Login"],
        ev(Ev::Submit, |event| {
            event.prevent_default();
            Login(Msg::Submit)
        }),
        div![
            C!("inputs"),
            input![
                C!("inputs"),
                attrs! { At::Value => model.form.name, At::Placeholder => "name" },
                input_ev(Ev::Input, |name| { Login(Msg::NameChanged(name)) }),
            ],
            br!(),
            br!(),
            input![
                attrs! { At::Value => model.form.password, At::Type => "password", At::Placeholder => "password" },
                input_ev(Ev::Input, |password| { Login(Msg::PasswordChanged(password)) }),
            ]
        ],
        div![C!("center-button"), button![C!["button"], "Login"]],
        IF!(model.text.is_some() => match model.text {
            Some(Status::Success(ref s)) => h2! [C!("success"), s],
            Some(Status::Error(ref s)) => h2! [C!("error"), s],
            _ => unreachable!()
        })
    ]
}
