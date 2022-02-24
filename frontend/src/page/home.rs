use seed::{prelude::*, *};
use shared::model::CreateGameChallenge;

pub struct Model
{
    available_games: Vec<CreateGameChallenge>,
}


pub fn init() -> Model
{
    Model {
        available_games: vec![CreateGameChallenge {
            creator: "Sivert".to_string(),
            uuid:    String::new(),
        }, CreateGameChallenge {
            creator: "Bernt".to_string(),
            uuid:    String::new(),
        }],
    }
}


pub fn view<Ms>(model: &Model) -> Node<Ms>
{
    div! [
        C!("container"),
        h1!["Available games!"],
        div! [  attrs! { At::Width => "50%" },
            C!("table-container"),
            table! [
                tr! [
                    th! [ "Challenger" ],
                    th![ "Accept" ],
                ],
                model.available_games.iter().map(|game| 
                    tr! [
                        td![ &game.creator ],
                        td![ button! [ C!("button accept-button"), "Accept" ] ]
                    ])
            ]
        ]
    ]
}
// .filter_map(|(label, flag)| flag.then(|| Some(option![attrs! {At::Value => label }])))
