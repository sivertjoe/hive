use seed::{self, prelude::*, *};
use shared::model::{http::*, UserCredentials};

use crate::Msg::Login;
use shared::ObjectId;

use crate::request::user_cred::*;

pub fn init(text: String, (end_point, success_code): (String, u32)) -> Model {
    Model {
        form: UserCredentials::default(),
        status_text: None,
        end_point,
        success_code,
        text,
    }
}

pub struct Model {
    form: UserCredentials,
    status_text: Option<Status>,
    success_code: u32,
    end_point: String,
    text: String,
}

enum Status {
    Success(String),
    Error(String),
}

pub enum Msg {
    Submit,
    NameChanged(String),
    PasswordChanged(String),
    Fetched(fetch::Result<String>),
}

pub fn update(
    msg: Msg,
    model: &mut Model,
    orders: &mut impl Orders<crate::Msg>,
    to_msg: impl FnOnce(Msg) -> crate::Msg + Clone + 'static,
) {
    match msg {
        Msg::Submit => {
            let form = model.form.clone();
            let end_point = model.end_point.clone();
            orders.skip().perform_cmd(async move {
                to_msg(Msg::Fetched(send_message(&end_point, form).await))
            });
        }
        Msg::NameChanged(name) => {
            model.form.name = name;
        }
        Msg::PasswordChanged(password) => {
            model.form.password = password;
        }

        Msg::Fetched(Ok(text)) => match serde_json::from_str::<ResponseBody>(&text) {
            Ok(resp) => match resp.status {
                code if code == model.success_code => {
                    model.status_text = Some(Status::Success("Success".into()));
                    let id: ObjectId = resp.get_body();
                    let name = &model.form.name;
                    LocalStorage::insert("id", &id).expect("inserting id in LocalStorage");
                    LocalStorage::insert("name", &name).expect("inserting name in LocalStorage");
                    orders.send_msg(Login {
                        name: model.form.name.clone(),
                    });
                }

                e => {
                    let err: String = resp.get_body();
                    model.status_text = Some(Status::Error(format!("Error ({e}): {err}")));
                }
            },
            Err(e) => {
                model.status_text = Some(Status::Error(format!("deserialize error, {e:?}")));
            }
        },

        Msg::Fetched(Err(text)) => {
            model.status_text = Some(Status::Error(format!("{text:?}")));
        }
    }
}

pub fn view(
    model: &Model,
    to_msg: impl Fn(Msg) -> crate::Msg + Clone + Copy + 'static,
) -> Node<crate::Msg> {
    form![
        C!["container"],
        h1![&model.text],
        ev(Ev::Submit, move |event| {
            event.prevent_default();
            to_msg(Msg::Submit)
        }),
        div![
            C!("inputs"),
            input![
                C!("inputs"),
                attrs! { At::Value => model.form.name, At::Placeholder => "name" },
                input_ev(Ev::Input, move |name| { to_msg(Msg::NameChanged(name)) }),
            ],
            br!(),
            br!(),
            input![
                attrs! { At::Value => model.form.password, At::Type => "password", At::Placeholder => "password" },
                input_ev(Ev::Input, move |password| {
                    to_msg(Msg::PasswordChanged(password))
                }),
            ]
        ],
        div![C!("center-button"), button![C!["button"], &model.text]],
        IF!(model.status_text.is_some() => match model.status_text {
            Some(Status::Success(ref s)) => h2! [C!("success"), s],
            Some(Status::Error(ref s)) => h2! [C!("error"), s],
            _ => unreachable!()
        })
    ]
}
