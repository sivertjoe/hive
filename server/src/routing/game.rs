use hyper::{Body, Method, Request, Response};
use mongodb::bson::oid::ObjectId;
use shared::model::Move;

use super::{bad_request, error, get_body, method_not_allowed, ok};
use crate::{
    database::{
        complete_game, get_active_games, get_game_by_id, get_old_games, get_users_games, play_move,
        DatabaseError::GameNotComplete,
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

            Query::Old =>
            {
                let res = get_old_games(state.db()).await.unwrap();
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

            Query::User(object_id) => match get_users_games(state.db(), object_id).await
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
    match play_move(state.db(), r#move.clone()).await
    {
        Ok(()) =>
        {
            let msg = crate::websocket::Message {
                r#move,
            };
            match state.tx.send(msg).await
            {
                Ok(_) => Response::new(ok(())),
                _ => panic!("websocket server is dead"),
            }
        },
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
    Old,
    Id(ObjectId),
    User(ObjectId),
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

        if s.starts_with("q=user")
        {
            // get the string s within "q=user(s)""
            let id = &s[7..s.len() - 1];
            match ObjectId::parse_str(id)
            {
                Ok(id) => Ok(Query::User(id)),
                _ => Err(()),
            }
        }
        else
        {
            match &s[2..]
            {
                "all" => Ok(Query::All),
                "old" => Ok(Query::Old),
                s => match ObjectId::parse_str(s)
                {
                    Ok(id) => Ok(Query::Id(id)),
                    _ => Err(()),
                },
            }
        }
    }
}
