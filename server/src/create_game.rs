use serde::{Deserialize, Serialize};
use shared::{model::CreateGameForm, Uuid};

use crate::database::uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGame
{
    pub uuid:    Uuid,
    pub creator: Uuid,
}

impl CreateGame
{
    pub fn from_form(form: CreateGameForm) -> Self
    {
        Self {
            creator: form.creator, uuid: uuid()
        }
    }
}
