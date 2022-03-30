use hyper::{Body, Method, Request, Response};
use shared::model::UserCredentials;

use super::{create, error, get_body, method_not_allowed};
use crate::{database::register_user, State};

pub async fn register(req: Request<Body>, state: State) -> Response<Body>
{
    match *req.method()
    {
        Method::POST =>
        {
            let cred = get_body::<UserCredentials>(req).await.unwrap();

            match register_user(state.db(), cred).await
            {
                Ok(uuid) => Response::new(create(uuid)),
                Err(e) => Response::new(error(e)),
            }
        },
        _ => Response::new(method_not_allowed()),
    }
}
