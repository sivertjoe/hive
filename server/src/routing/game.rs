use hyper::{Body, Method, Request, Response};
use mongodb::Client;
use shared::{model::UserCredentials, Uuid};

use super::{bad_request, error, get_body, method_not_allowed, ok};
use crate::database::{self, get_active_games, LIVE};

pub async fn game(req: Request<Body>, client: Client) -> Response<Body>
{
    let uri = req.uri();
    // TODO
    match *req.method()
    {
        Method::GET => match req.uri().query()
        {
            Some("q=all") =>
            {
                let res = get_active_games(client.database(LIVE)).await.unwrap();
                Response::new(ok(res))
            },
            _ => Response::new(bad_request()),
        },
        _ => Response::new(method_not_allowed()),
    }
}
