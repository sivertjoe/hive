use hyper::{Body, Method, Request};
use shared::ObjectId;

use super::{data, HttpError, HttpResult};
use crate::{database::register_user, State};


pub async fn register(req: Request<Body>, state: State) -> HttpResult<ObjectId>
{
    match *req.method()
    {
        Method::POST => data(register_user, req, state.db(), HttpResult::Create).await,
        _ => HttpResult::Err(HttpError::MethodNotAllowed),
    }
}
