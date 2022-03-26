pub mod create;
pub mod game;
pub mod home;
pub mod user_cred;

use const_format::formatcp;

const PORT: usize = 5000;
const PROD_API_URL: &str = "https://hive.sivert.dev";

const BASE_API_URL: &str = formatcp!(
    "{HOST}:{PORT}/api",
    HOST = if cfg!(debug_assertions) {
        "http://0.0.0.0"
    } else {
        PROD_API_URL
    }
);

fn url(end_point: &str) -> String {
    format!("{}/{}", BASE_API_URL, end_point)
}
