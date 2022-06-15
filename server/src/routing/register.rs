use hyper::{Body, Method, Request};

use super::{get_body, HttpError, HttpResult};
use crate::{database::register_user, State};


/*
 * Function for registering user to the site.
 * The functions expects a UserCredentials struct inside the request.
 * Returns the ObjectId of the newly registered user on success.
 */
pub async fn register(req: Request<Body>, state: State) -> HttpResult
{
    match *req.method()
    {
        Method::POST => match get_body(req).await
        {
            Some(cred) => match register_user(state.db(), cred).await
            {
                Ok(id) => HttpResult::new(HttpResult::Create, id),
                Err(e) => HttpResult::Err(HttpError::Database(e)),
            },

            None => HttpResult::Err(HttpError::Serialize),
        },
        _ => HttpResult::Err(HttpError::MethodNotAllowed),
    }
}
