pub mod create;
pub mod game;
pub mod home;
pub mod user_cred;

use crate::ObjectId;
use const_format::formatcp;

const PORT: usize = 5000;
const WEB_SOCKET_PORT: usize = 5001;
const PROD_URL: &str = if cfg!(debug_assertions) {
    "0.0.0.0"
} else {
    "hive.sivert.dev"
};

const PROTOCOL: &str = if cfg!(debug_assertions) {
    "http"
} else {
    "https"
};

const BASE_API_URL: &str = formatcp!("{PROTOCOL}://{PROD_URL}/api",);

fn url(end_point: &str) -> String {
    format!("{}/{}", BASE_API_URL, end_point)
}

const WEB_SOCKET_ULR: &str = formatcp!("wss://{PROD_URL}/ws");

pub fn ws_url(game_id: ObjectId) -> String {
    dbg!(format!("{}?id={}", WEB_SOCKET_ULR, game_id))
}
