use hyper::{Body, Method, Request};

use super::{get_body, HttpError, HttpResult};
use crate::{database, State};


/*
 * Function for logging in a user to the site.
 * The functions expects a UserCredentials struct inside the request.
 * The function checks whether a user is registered with said credentails
 * and returns the object id on success.
 */
pub async fn login(req: Request<Body>, state: State) -> HttpResult
{
    match *req.method()
    {
        Method::POST => match get_body(req).await
        {
            Some(cred) => match database::login(state.db(), cred).await
            {
                Ok(id) => HttpResult::new(HttpResult::Ok, id),
                Err(e) => HttpResult::Err(HttpError::Database(e)),
            },
            None => HttpResult::Err(HttpError::Serialize),
        },
        _ => HttpResult::Err(HttpError::MethodNotAllowed),
    }
}
