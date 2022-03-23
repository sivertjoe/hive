use super::url;

use seed::prelude::*;
use shared::ObjectId;

pub async fn send_create_request(creator: ObjectId) -> fetch::Result<String> {
    Request::new(url("create-game"))
        .method(Method::Post)
        .json(&creator)?
        .fetch()
        .await?
        .check_status()?
        .text()
        .await
}
