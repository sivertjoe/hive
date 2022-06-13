use seed::{prelude::*, *};
use shared::model::{
    CreateGameChallenge, CreateGameChallengeBundle, CreateGameFormResponse, OnGoingGame,
    ResponseBody,
};
use shared::ObjectId;

use crate::request::home::*;
use crate::Msg::Home;

#[derive(Default)]
pub struct Model {
    available_games: Vec<CreateGameChallenge>,
    label: Option<String>,
    ongoing_games: Vec<OnGoingGame>,
    old_games: Vec<OnGoingGame>,   // Shh
    users_games: Vec<OnGoingGame>, // Shh
}

fn is_logged_in() -> bool {
    let f: seed::browser::web_storage::Result<ObjectId> = LocalStorage::get("id");
    f.is_ok()
}

pub fn init(orders: &mut impl Orders<Msg>) -> Model {
    orders
        .skip()
        .perform_cmd(async { Msg::FetchedAvailableGames(get_all_games().await) })
        .perform_cmd(async { Msg::FetchedOldGames(get_old_games().await) })
        .perform_cmd(async {
            let id: Result<ObjectId, _> = LocalStorage::get("id");
            if let Ok(id) = id {
                Msg::FetchedCreateGame(send_message(id, "home", Method::Post).await)
            } else {
                Msg::FetchedCreateGame(send_message("noid", "home", Method::Post).await)
            }
        })
        .perform_cmd(async {
            let id: Result<ObjectId, _> = LocalStorage::get("id");
            if let Ok(id) = id {
                Msg::FetchedUsersGames(get_users_games(id.to_string()).await)
            } else {
                Msg::FetchedUsersGames(get_users_games("noid".to_string()).await)
            }
        });

    Model::default()
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
                creator: creator_id,
            })
        })
        .collect()
}

pub enum Msg {
    FetchedCreateGame(fetch::Result<String>),
    AcceptGame { game: ObjectId, creator: ObjectId },
    AcceptedGame(fetch::Result<String>),

    FetchedAvailableGames(fetch::Result<String>),
    FetchedOldGames(fetch::Result<String>),
    FetchedUsersGames(fetch::Result<String>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    let parse_resp = |resp: fetch::Result<String>| -> Result<ResponseBody, String> {
        resp.map_err(|e| format!("{e:?}"))
            .and_then(|text| {
                serde_json::from_str::<ResponseBody>(&text).map_err(|e| format!("{e:?}"))
            })
            .and_then(|resp| {
                if resp.status == 200 {
                    Ok(resp)
                } else {
                    Err("wrong statuscode".into())
                }
            })
    };

    match msg {
        Msg::FetchedOldGames(text) => match parse_resp(text) {
            Ok(resp) => {
                let games: Vec<OnGoingGame> = resp.get_body();
                model.old_games = games;
            }
            Err(e) => model.label = Some(e),
        },

        Msg::FetchedUsersGames(text) => match parse_resp(text) {
            Ok(resp) => {
                let games: Vec<OnGoingGame> = resp.get_body();
                model.users_games = games;
            }
            Err(_) => {}
        },

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
                        game_object_id: accept.object_id,
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

fn challenge<Ms: 'static>(game: &CreateGameChallenge) -> Node<Ms> {
    let creator = game.creator;
    let game = game._id;
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
            thead![tr![th!["Challenger"], th!["Accept"]]],
            tbody![
                model
                    .available_games
                    .iter()
                    .map(|game| { tr![td![&game.name], td![challenge(game)]] })
            ]
        ]
    ]
}

fn ongoing_game<Ms: 'static>(game: &OnGoingGame) -> Node<Ms> {
    let id = game.game_object_id.to_string();
    let url = Url::new().add_path_part("game").add_path_part(&id);
    tr![
        ev(Ev::Click, move |_| {
            url.go_and_load();
        }),
        td![&game.players[0]],
        td![&game.players[1]],
    ]
}

fn ongoing_games<Ms: 'static>(model: &Model) -> Node<Ms> {
    div![
        h1!["Ongoing games!"],
        table![
            C!("challenge-table"),
            thead![tr![th!["White"], th!["Black"]]],
            tbody![
                model
                    .ongoing_games
                    .iter()
                    .map(|game| { ongoing_game(game) })
            ]
        ]
    ]
}

fn old_games<Ms: 'static>(model: &Model) -> Node<Ms> {
    div![
        h1!["Old games!"],
        table![
            C!("challenge-table"),
            thead![tr![th!["White"], th!["Black"]]],
            tbody![model.old_games.iter().map(|game| { ongoing_game(game) })]
        ]
    ]
}

fn users_games<Ms: 'static>(model: &Model) -> Node<Ms> {
    div![
        h1!["Your games!"],
        table![
            C!("challenge-table"),
            thead![tr![th!["White"], th!["Black"]]],
            tbody![model.users_games.iter().map(|game| { ongoing_game(game) })]
        ]
    ]
}

fn label<Ms: 'static>(model: &Model) -> Option<Node<Ms>> {
    IF!(model.label.is_some() => match model.label {
        Some(ref s) => h2! [C!("error"), s],
        _ => unreachable!()
    })
}

pub fn view<Ms: 'static>(model: &Model) -> Node<Ms> {
    let logged_in = is_logged_in();
    div![
        C!("container"),
        div![
            C!("grid-container"),
            IF!(logged_in =>
                available_games(model)
            ),
            ongoing_games(model),
            IF!(logged_in =>
                users_games(model)
            ),
            old_games(model)
        ],
        label(model),
    ]
}
