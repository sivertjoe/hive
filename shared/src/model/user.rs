use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UserCredentials
{
    pub name:     String,
    pub password: String,
}
