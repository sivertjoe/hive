use hyper::{Body, Method, Request, Response};
use mongodb::Client;
use shared::{model::UserCredentials, Uuid};

use super::{error, get_body, method_not_allowed, ok};
use crate::database::{self, LIVE};

pub async fn home(req: Request<Body>, client: Client) -> Response<Body>
{
    match *req.method()
    {
        Method::POST =>
        {
            let uuid = get_body::<Uuid>(req).await.unwrap();

            match database::home(client.database(LIVE), uuid).await
            {
                Ok(uuid) => Response::new(ok(uuid)),
                Err(e) => Response::new(error(e)),
            }
        },
        _ => Response::new(method_not_allowed()),
    }
}
