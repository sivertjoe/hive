use serde::{Deserialize, Serialize};

type HttpStatusCode = u32;
#[derive(Serialize, Deserialize)]
pub struct ResponseBody
{
    pub status: HttpStatusCode,
    pub body:   String,
}

impl ResponseBody
{
    pub fn to_body(status: HttpStatusCode, body: String) -> String
    {
        let resp = Self {
            status,
            body,
        };

        serde_json::to_string(&resp).unwrap()
    }

    pub fn get_body<T>(self) -> T
    where
        T: serde::de::DeserializeOwned,
    {
        serde_json::from_str(self.body.as_str()).unwrap()
    }
}
