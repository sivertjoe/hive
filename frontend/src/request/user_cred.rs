use super::url;
use seed::prelude::*;
use shared::model::UserCredentials;

pub async fn send_message(end_point: &str, form: UserCredentials) -> fetch::Result<String> {
    Request::new(url(end_point))
        .method(Method::Post)
        .json(&form)?
        .fetch()
        .await?
        .check_status()?
        .text()
        .await
}
