use hyper::{Body, Method, Request};

use super::{get_body, HttpError, HttpResult};
use crate::{database, State};


/*
 * Function for handling game invites:
 *     POST: create a new game invite. Takes an AcceptGame struct in the body
 *     PUT : Accept a game invite.     Takes the ObjectId of the invite in
 * the body.
 */
pub async fn create_game(req: Request<Body>, state: State) -> HttpResult
{
    match *req.method()
    {
        Method::PUT => match get_body(req).await
        {
            Some(ag) => match database::accept_game(state.db(), ag).await
            {
                Ok(res) => HttpResult::new(HttpResult::Ok, res),
                Err(e) => HttpResult::Err(HttpError::Database(e)),
            },
            None => HttpResult::Err(HttpError::Serialize),
        },
        Method::POST => match get_body(req).await
        {
            Some(ag) => match database::create_game(state.db(), ag).await
            {
                Ok(res) => HttpResult::new(HttpResult::Ok, res),
                Err(e) => HttpResult::Err(HttpError::Database(e)),
            },
            None => HttpResult::Err(HttpError::Serialize),
        },
        _ => HttpResult::Err(HttpError::MethodNotAllowed),
    }
}
