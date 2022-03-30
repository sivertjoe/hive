use hyper::{Body, Method, Request, Response};
use mongodb::bson::oid::ObjectId;
use shared::model::Move;

use super::{bad_request, error, get_body, method_not_allowed, ok};
use crate::{
    database::{
        complete_game, get_active_games, get_game_by_id, play_move, DatabaseError::GameNotComplete,
    },
    State,
};


async fn get(req: Request<Body>, state: State) -> Response<Body>
{
    match req.uri().query().and_then(|uri| Query::from_str(uri).ok())
    {
        Some(q) => match q
        {
            Query::All =>
            {
                let res = get_active_games(state.db()).await.unwrap();
                Response::new(ok(res))
            },
            Query::Id(object_id) => match get_game_by_id(state.db(), object_id).await
            {
                Ok(res) => Response::new(ok(res)),

                Err(e) =>
                {
                    println!("err");
                    Response::new(error(e))
                },
            },
        },
        _ => Response::new(bad_request()),
    }
}


async fn post(req: Request<Body>, state: State) -> Response<Body>
{
    let r#move = get_body::<Move>(req).await.unwrap();
    match play_move(state.db(), r#move).await
    {
        Ok(()) => Response::new(ok(())),
        Err(e) => Response::new(error(e)),
    }
}


pub async fn game(req: Request<Body>, state: State) -> Response<Body>
{
    match *req.method()
    {
        Method::GET => get(req, state).await,

        Method::POST => post(req, state).await,

        Method::DELETE =>
        {
            let id = get_body::<ObjectId>(req).await.unwrap();

            let db = state.db();
            let game = match get_game_by_id(db.clone(), id).await
            {
                Ok(game) => game,
                Err(e) => return Response::new(error(e)),
            };

            if game.board.is_complete()
            {
                complete_game(db, id).await.unwrap();
                Response::new(ok(()))
            }
            else
            {
                Response::new(error(GameNotComplete))
            }
        },
        _ => Response::new(method_not_allowed()),
    }
}

enum Query
{
    All,
    Id(ObjectId),
}

use std::str::FromStr;
impl FromStr for Query
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        // HAS to start with q= atleast
        if s.len() <= 2
        {
            return Err(());
        }

        match &s[2..]
        {
            "all" => Ok(Query::All),
            s => match ObjectId::parse_str(s)
            {
                Ok(id) => Ok(Query::Id(id)),
                _ => Err(()),
            },
        }
    }
}
