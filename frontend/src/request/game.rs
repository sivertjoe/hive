use super::url;
use seed::prelude::*;
use shared::model::Move;
use shared::ObjectId;

pub async fn get_game(id: ObjectId) -> fetch::Result<String> {
    Request::new(url(&format!("game?q={id}")))
        .method(Method::Get)
        .fetch()
        .await?
        .check_status()?
        .text()
        .await
}

pub async fn send_move(r#move: Move) -> fetch::Result<String> {
    Request::new(url(&format!("game?q={id}", id = r#move.player_id)))
        .method(Method::Post)
        .json(&r#move)?
        .fetch()
        .await?
        .check_status()?
        .text()
        .await
}

pub async fn complete_game(id: ObjectId) -> fetch::Result<String> {
    Request::new(url("game"))
        .method(Method::Delete)
        .json(&id)?
        .fetch()
        .await?
        .check_status()?
        .text()
        .await
}
