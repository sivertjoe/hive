use super::url;
use seed::prelude::*;
use serde::Serialize;
// use shared::ObjectId;

pub async fn get_all_games() -> fetch::Result<String> {
    Request::new(url("game?q=all"))
        .method(Method::Get)
        .fetch()
        .await?
        .check_status()?
        .text()
        .await
}

pub async fn get_old_games() -> fetch::Result<String> {
    Request::new(url("game?q=old"))
        .method(Method::Get)
        .fetch()
        .await?
        .check_status()?
        .text()
        .await
}

pub async fn get_users_games(id: String) -> fetch::Result<String> {
    Request::new(url(&format!("game?q=user({id})")))
        .method(Method::Get)
        .fetch()
        .await?
        .check_status()?
        .text()
        .await
}

pub async fn send_message<T>(body: T, end_point: &str, method: Method) -> fetch::Result<String>
where
    T: Serialize,
{
    Request::new(url(end_point))
        .method(method)
        .json(&body)?
        .fetch()
        .await?
        .check_status()?
        .text()
        .await
}
