use seed::{prelude::*, *};
use shared::model::{CreateGameChallenge, CreateGameChallengeBundle, ResponseBody};

pub struct Model {
    available_games: Vec<CreateGameChallenge>,
}

fn challenge_from_bundle(bundle: Vec<CreateGameChallengeBundle>) -> Vec<CreateGameChallenge> {
    bundle
        .into_iter()
        .flat_map(|user| {
            let CreateGameChallengeBundle { name, games } = user;

            games.into_iter().map(move |game_uuid| CreateGameChallenge {
                name: name.clone(),
                uuid: game_uuid,
            })
        })
        .collect()
}

pub enum Msg {
    FetchedCreateGame(fetch::Result<String>),
}

pub fn update(msg: Msg, model: &mut Model) {
    match msg {
        Msg::FetchedCreateGame(Ok(text)) => match serde_json::from_str::<ResponseBody>(&text) {
            Ok(resp) => match resp.status {
                200 => {
                    let vec = resp.get_body::<Vec<CreateGameChallengeBundle>>();
                }
                e => {
                    // handle..
                }
            },
            Err(e) => {
                // handle
            }
        },

        Msg::FetchedCreateGame(Err(text)) => {}
    }
}

async fn send_message() -> fetch::Result<String> {
    let uuid = LocalStorage::get("uuid").unwrap_or_else(|_| String::new());
    Request::new(format!("http://0.0.0.0:5000/home"))
        .method(Method::Post)
        .json(&uuid)?
        .fetch()
        .await?
        .check_status()?
        .text()
        .await
}

pub fn init(orders: &mut impl Orders<Msg>) -> Model {
    orders
        .skip()
        .perform_cmd(async { Msg::FetchedCreateGame(send_message().await) });

    Model {
        available_games: Vec::new(),
    }
}

pub fn view<Ms>(model: &Model) -> Node<Ms> {
    div![
        C!("container"),
        h1!["Available games!"],
        table![
            C!("challenge-table"),
            tr![th!["Challenger"], th!["Accept"],],
            model.available_games.iter().map(|game| tr![
                td![&game.name],
                td![button![C!("button accept-button"), "Accept"]]
            ])
        ]
    ]
}
