use hyper::{Body, Method, Request, Response};
use mongodb::Client;
use shared::model::CreateGameForm;

use super::{create, error, get_body, method_not_allowed};
use crate::database::{create_game, LIVE};

pub async fn game(req: Request<Body>, client: Client) -> Response<Body>
{
    match *req.method()
    {
        Method::POST =>
        {
            let form = get_body::<CreateGameForm>(req).await.unwrap();

            match create_game(client.database(LIVE), form).await
            {
                Ok(()) => Response::new(create(())),
                Err(e) => Response::new(error(e)),
            }
        },
        _ => Response::new(method_not_allowed()),
    }
}
