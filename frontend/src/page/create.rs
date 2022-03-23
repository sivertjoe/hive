use seed::{prelude::*, *};
use shared::model::http::*;

use crate::request::create::*;
use crate::Msg::CreateGame;

pub fn init() -> Model {
    Model::default()
}

pub enum Msg {
    Submit,
    Fetched(fetch::Result<String>),
}

#[derive(Default)]
pub struct Model {
    text: Option<Status>,
}

enum Status {
    Success(String),
    Error(String),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Submit => match LocalStorage::get("id") {
            Ok(id) => {
                orders
                    .skip()
                    .perform_cmd(async move { Msg::Fetched(send_create_request(id).await) });
            }
            Err(_) => {
                model.text = Some(Status::Error("User not logged in".into()));
            }
        },
        Msg::Fetched(Ok(text)) => match serde_json::from_str::<ResponseBody>(&text) {
            Ok(resp) => match resp.status {
                201 => {
                    model.text = Some(Status::Success("Game successfully created!".into()));
                }

                e => {
                    model.text = Some(Status::Error(format!("Error ({e})")));
                }
            },
            Err(e) => {
                model.text = Some(Status::Error(format!("deserialize error, {e:?}")));
            }
        },

        Msg::Fetched(Err(text)) => {
            model.text = Some(Status::Error(format!("{text:?}")));
        }
    }
}

pub fn view<Ms: 'static>(model: &Model) -> Node<Ms> {
    let body = || {
        form![
            ev(Ev::Submit, |event| {
                event.prevent_default();
                CreateGame(Msg::Submit)
            }),
            div![C!("center-button"), button![C!["button"], "Create"]],
        ]
    };
    div![
        C!("container center"),
        body(),
        IF!(model.text.is_some() => match model.text {
            Some(Status::Success(ref s)) => h2! [C!("success"), s],
            Some(Status::Error(ref s)) => h2! [C!("error"), s],
            _ => unreachable!()
        })
    ]
}
