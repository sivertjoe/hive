use seed::{prelude::*, *};
use serde::Serialize;
use shared::model::{
    CreateGameChallenge, CreateGameChallengeBundle, CreateGameFormResponse, OnGoingGame,
    ResponseBody,
};
use shared::ObjectId;

use crate::Msg::Home;
pub struct Model {
    available_games: Vec<CreateGameChallenge>,
    label: Option<String>,
    ongoing_games: Vec<OnGoingGame>,
}

pub fn init(orders: &mut impl Orders<Msg>) -> Model {
    orders
        .skip()
        .perform_cmd(async { Msg::FetchedAvailableGames(get_all_games().await) })
        .perform_cmd(async {
            let id = LocalStorage::get("id").unwrap_or_else(|_| String::new());
            Msg::FetchedCreateGame(send_message(id, "home", Method::Post).await)
        });

    Model {
        available_games: Vec::new(),
        label: None,
        ongoing_games: Vec::new(),
    }
}

fn challenge_from_bundle(bundle: Vec<CreateGameChallengeBundle>) -> Vec<CreateGameChallenge> {
    bundle
        .into_iter()
        .flat_map(|user| {
            let CreateGameChallengeBundle {
                name,
                games,
                creator_id,
            } = user;

            games.into_iter().map(move |game_id| CreateGameChallenge {
                name: name.clone(),
                _id: game_id,
                creator: creator_id.clone(),
            })
        })
        .collect()
}

pub enum Msg {
    FetchedCreateGame(fetch::Result<String>),
    AcceptGame { game: ObjectId, creator: ObjectId },
    AcceptedGame(fetch::Result<String>),
    FetchedAvailableGames(fetch::Result<String>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FetchedCreateGame(Ok(text)) => match serde_json::from_str::<ResponseBody>(&text) {
            Ok(resp) => match resp.status {
                200 => {
                    let bundle = resp.get_body::<Vec<CreateGameChallengeBundle>>();
                    model.available_games = challenge_from_bundle(bundle);
                }
                e => {
                    model.label = Some(format!("error: {e}"));
                }
            },
            Err(e) => {
                model.label = Some(format!("error: {e:?}"));
            }
        },

        Msg::FetchedCreateGame(Err(text)) => {
            model.label = Some(format!("error: {text:?}"));
        }

        Msg::AcceptGame { game, creator } => match LocalStorage::get("id").ok() {
            Some(id) => {
                let form = CreateGameFormResponse {
                    game,
                    creator,
                    user: id,
                };

                orders.skip().perform_cmd(async {
                    Msg::AcceptedGame(send_message(form, "create-game", Method::Put).await)
                });
            }
            _ => {
                model.label = Some("must be logged in".into());
            }
        },

        Msg::AcceptedGame(Ok(text)) => match serde_json::from_str::<ResponseBody>(&text) {
            Ok(resp) => match resp.status {
                201 => {
                    let accept: shared::model::AcceptGame = resp.get_body();
                    let idx = model
                        .available_games
                        .iter()
                        .position(|g| g._id == accept.game)
                        .unwrap();

                    let game = model.available_games.remove(idx);
                    let name = LocalStorage::get("name").unwrap();
                    model.ongoing_games.push(OnGoingGame {
                        game_object_id: accept.object_id.to_string(),
                        players: [game.name, name],
                    });
                }
                e => {
                    model.label = Some(format!("error: {e}"));
                }
            },
            Err(e) => {
                model.label = Some(format!("error: {e:?}"));
            }
        },
        Msg::AcceptedGame(Err(text)) => {
            model.label = Some(format!("error: {text:?}"));
        }

        Msg::FetchedAvailableGames(Ok(text)) => match serde_json::from_str::<ResponseBody>(&text) {
            Ok(resp) => match resp.status {
                200 => {
                    let games: Vec<OnGoingGame> = resp.get_body();
                    model.ongoing_games = games;
                }
                e => {
                    model.label = Some(format!("error: {e}"));
                }
            },
            Err(e) => {
                model.label = Some(format!("error: {e:?}"));
            }
        },
        Msg::FetchedAvailableGames(Err(text)) => {
            model.label = Some(format!("error: {text:?}"));
        }
    }
}

async fn get_all_games() -> fetch::Result<String> {
    Request::new(format!("http://0.0.0.0:5000/game?q=all"))
        .method(Method::Get)
        .fetch()
        .await?
        .check_status()?
        .text()
        .await
}

async fn send_message<T>(body: T, end_point: &str, method: Method) -> fetch::Result<String>
where
    T: Serialize,
{
    Request::new(format!("http://0.0.0.0:5000/{end_point}"))
        .method(method)
        .json(&body)?
        .fetch()
        .await?
        .check_status()?
        .text()
        .await
}

fn challenge<Ms: 'static>(game: &CreateGameChallenge) -> Node<Ms> {
    let creator = game.creator.clone();
    let game = game._id.clone();
    button![
        C!("button accept-button"),
        "Accept",
        ev(Ev::Click, move |event| {
            event.prevent_default();

            Home(Msg::AcceptGame { creator, game })
        })
    ]
}

fn available_games<Ms: 'static>(model: &Model) -> Node<Ms> {
    div![
        h1!["Available Games!"],
        table![
            C!("challenge-table"),
            tr![th!["Challenger"], th!["Accept"],],
            model
                .available_games
                .iter()
                .map(|game| { tr![td![&game.name], td![challenge(game)]] })
        ]
    ]
}

fn ongoing_game<Ms: 'static>(game: &OnGoingGame) -> Node<Ms> {
    let id = game.game_object_id.clone();
    let url = Url::new().add_path_part("game").add_path_part(&id);
    tr![
        a![
            attrs! { At::Href => url },
            td![attrs! { At::Width => "50%" }, &game.players[0]]
        ],
        td![attrs! { At::Width => "50%" }, &game.players[1]],
    ]
}

fn ongoing_games<Ms: 'static>(model: &Model) -> Option<Node<Ms>> {
    IF!(!model.ongoing_games.is_empty() => {
        div![
    h1!["Ongoing Games!"],
    table![
        C!("challenge-table"),
        model
            .ongoing_games
            .iter()
            .map(|game| { ongoing_game(game) })
    ]
        ]
    })
}

fn label<Ms: 'static>(model: &Model) -> Option<Node<Ms>> {
    IF!(model.label.is_some() => match model.label {
        Some(ref s) => h2! [C!("error"), s],
        _ => unreachable!()
    })
}

pub fn view<Ms: 'static>(model: &Model) -> Node<Ms> {
    div![
        C!("container"),
        available_games(model),
        br!(),
        br!(),
        ongoing_games(model),
        label(model),
    ]
}
