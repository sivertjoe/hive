use hyper::{Body, Method, Request};
use shared::ObjectId;

use super::{data, HttpError, HttpResult};
use crate::{database, State};


pub async fn login(req: Request<Body>, state: State) -> HttpResult<ObjectId>
{
    match *req.method()
    {
        Method::POST => data(database::login, req, state.db(), HttpResult::Ok).await,
        _ => HttpResult::Err(HttpError::MethodNotAllowed),
    }
}
