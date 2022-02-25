use seed::{prelude::*, *};
use shared::model::CreateGameChallenge;

pub struct Model {
    available_games: Vec<CreateGameChallenge>,
}

pub fn init() -> Model {
    Model {
        available_games: vec![
            CreateGameChallenge {
                creator: "Sivert".to_string(),
                uuid: String::new(),
            },
            CreateGameChallenge {
                creator: "Bernt".to_string(),
                uuid: String::new(),
            },
        ],
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
                td![&game.creator],
                td![button![C!("button accept-button"), "Accept"]]
            ])
        ]
    ]
}
