use hyper::{Body, Method, Request, Response};
use mongodb::bson::oid::ObjectId;

use super::{error, get_body, method_not_allowed, ok};
use crate::{database, State};

pub async fn home(req: Request<Body>, state: State) -> Response<Body>
{
    match *req.method()
    {
        Method::POST =>
        {
            // Just generate and use a new ObjectId if none was received, i.e,
            // user was not logged in
            let uuid = get_body::<ObjectId>(req).await.unwrap_or_else(ObjectId::new);

            match database::home(state.db(), uuid).await
            {
                Ok(bundle) => Response::new(ok(bundle)),
                Err(e) => Response::new(error(e)),
            }
        },
        _ => Response::new(method_not_allowed()),
    }
}
