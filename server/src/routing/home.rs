use hyper::{Body, Method, Request};
use mongodb::bson::oid::ObjectId;

use super::{get_body, HttpError, HttpResult};
use crate::{database, State};

/*
 * Function for getting home page information.
 * Takes the ObjectId of the user and returns available game invites.
 * Probably going do remove this at some point.
 */
pub async fn home(req: Request<Body>, state: State) -> HttpResult
{
    match *req.method()
    {
        Method::POST =>
        {
            // Just generate and use a new ObjectId if none was received, i.e,
            // user was not logged in
            let object_id = get_body::<ObjectId>(req).await.unwrap_or_else(ObjectId::new);

            match database::home(state.db(), object_id).await
            {
                Ok(bundle) => HttpResult::new(HttpResult::Ok, bundle),
                Err(e) => HttpResult::Err(HttpError::Database(e)),
            }
        },
        _ => HttpResult::Err(HttpError::MethodNotAllowed),
    }
}
