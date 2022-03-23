use super::url;
use seed::prelude::*;
use serde::Serialize;

pub async fn get_all_games() -> fetch::Result<String> {
    Request::new(url("game?q=all"))
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
    //Request::new(format!("http://0.0.0.0:5000/{end_point}"))
    Request::new(url(end_point))
        .method(method)
        .json(&body)?
        .fetch()
        .await?
        .check_status()?
        .text()
        .await
}
