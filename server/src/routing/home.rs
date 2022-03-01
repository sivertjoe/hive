use hyper::{Body, Method, Request, Response};
use mongodb::{bson::oid::ObjectId, Client};

use super::{error, get_body, method_not_allowed, ok};
use crate::database::{self, LIVE};

pub async fn home(req: Request<Body>, client: Client) -> Response<Body>
{
    match *req.method()
    {
        Method::POST =>
        {
            // Just generate and use a new ObjectId if none was received, i.e,
            // user was not logged in
            let uuid = get_body::<ObjectId>(req).await.unwrap_or_else(|| ObjectId::new());

            match database::home(client.database(LIVE), uuid).await
            {
                Ok(uuid) => Response::new(ok(uuid)),
                Err(e) => Response::new(error(e)),
            }
        },
        _ => Response::new(method_not_allowed()),
    }
}
