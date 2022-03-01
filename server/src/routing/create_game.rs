use hyper::{Body, Method, Request, Response};
use mongodb::Client;
use shared::{model::CreateGameFormResponse, ObjectId};

use super::{create, error, get_body, method_not_allowed};
use crate::database::{self, LIVE};

pub async fn create_game(req: Request<Body>, client: Client) -> Response<Body>
{
    match *req.method()
    {
        Method::POST =>
        {
            let id = get_body::<ObjectId>(req).await.unwrap();

            match database::create_game(client.database(LIVE), id).await
            {
                Ok(()) => Response::new(create(())),
                Err(e) => Response::new(error(e)),
            }
        },
        Method::PUT =>
        {
            let form = get_body::<CreateGameFormResponse>(req).await.unwrap();

            match database::accept_game(client.database(LIVE), form).await
            {
                Ok(id) => Response::new(create(id)),
                Err(e) => Response::new(error(e)),
            }
        },
        _ => Response::new(method_not_allowed()),
    }
}
