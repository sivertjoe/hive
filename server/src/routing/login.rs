use hyper::{Body, Method, Request, Response};
use mongodb::Client;
use shared::model::UserCredentials;

use super::{error, get_body, method_not_allowed, ok};
use crate::database::{self, LIVE};

pub async fn login(req: Request<Body>, client: Client) -> Response<Body>
{
    match *req.method()
    {
        Method::POST =>
        {
            let cred = get_body::<UserCredentials>(req).await.unwrap();

            match database::login(client.database(LIVE), cred).await
            {
                Ok(uuid) => Response::new(ok(uuid)),
                Err(e) => Response::new(error(e)),
            }
        },
        _ => Response::new(method_not_allowed()),
    }
}
