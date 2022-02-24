use serde::{Deserialize, Serialize};

use crate::Uuid;

#[derive(Serialize, Deserialize)]
pub struct CreateGameForm
{
    pub creator: Uuid,
}
