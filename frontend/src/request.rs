pub mod create;
pub mod game;
pub mod home;
pub mod user_cred;

use crate::ObjectId;
use const_format::formatcp;

const PORT: usize = 5000;
const WEB_SOCKET_PORT: usize = 5001;
/*const URL: &str = if cfg!(debug_assertions) {
    "localhost"
} else {
    "hive.sivert.dev"
};*/
const URL: &str = "hive.sivert.dev";

/*const BASE_API_URL: &str = if cfg!(debug_assertions) {
    formatcp!("http://{URL}:{PORT}/api")
} else {
    formatcp!("https://{URL}/api")
};*/

const BASE_API_URL: &str = formatcp!("https://{URL}/api");

fn url(end_point: &str) -> String {
    format!("{}/{}", BASE_API_URL, end_point)
}

const WEB_SOCKET_ULR: &str = if cfg!(debug_assertions) {
    formatcp!("ws://{URL}:{WEB_SOCKET_PORT}/ws")
} else {
    formatcp!("wss://{URL}/ws")
};

pub fn ws_url(game_id: ObjectId) -> String {
    dbg!(format!("{}?id={}", WEB_SOCKET_ULR, game_id))
}
