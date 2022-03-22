use hyper::{Body, Method, Request, Response};
use mongodb::{bson::oid::ObjectId, Client};
use shared::model::Move;

use super::{bad_request, error, get_body, method_not_allowed, ok};
use crate::database::{get_active_games, get_game_by_id, play_move, LIVE};


enum Query
{
    All,
    Id(ObjectId),
}

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


use std::str::FromStr;

pub async fn game(req: Request<Body>, client: Client) -> Response<Body>
{
    match *req.method()
    {
        Method::GET => match req.uri().query().and_then(|uri| Query::from_str(uri).ok())
        {
            Some(q) => match q
            {
                Query::All =>
                {
                    let res = get_active_games(client.database(LIVE)).await.unwrap();
                    Response::new(ok(res))
                },
                Query::Id(object_id) =>
                {
                    match get_game_by_id(client.database(LIVE), object_id).await
                    {
                        Ok(res) => Response::new(ok(res)),

                        Err(e) =>
                        {
                            println!("err");
                            Response::new(error(e))
                        },
                    }
                },
            },
            _ => Response::new(bad_request()),
        },

        Method::POST =>
        {
            let r#move = get_body::<Move>(req).await.unwrap();
            match play_move(client.database(LIVE), r#move).await
            {
                Ok(()) => Response::new(ok(())),
                Err(e) => Response::new(error(e)),
            }

            // Response::new(ok(()))
        },
        _ => Response::new(method_not_allowed()),
    }
}
