use hyper::{Body, Method, Request};
use shared::ObjectId;

use super::{data, HttpError, HttpResult};
use crate::{database, State};


pub async fn create_game(
    req: Request<Body>,
    state: State,
) -> HttpResult<Box<dyn erased_serde::Serialize>>
{
    match *req.method()
    {
        Method::PUT =>
        {
            let f = |acc| -> HttpResult<Box<dyn erased_serde::Serialize>> {
                HttpResult::Ok(Box::new(acc))
            };
            data(database::accept_game, req, state.db(), f).await
        },
        Method::POST =>
        {
            let f = |id: ObjectId| -> HttpResult<Box<dyn erased_serde::Serialize>> {
                HttpResult::Ok(Box::new(id))
            };

            data(database::create_game, req, state.db(), f).await
        },
        _ => HttpResult::Err(HttpError::MethodNotAllowed),
    }
}
