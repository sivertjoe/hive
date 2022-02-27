use std::convert::Infallible;

use hyper::{body, Body, Method, Request, Response};
use shared::model::http::*;

mod create_game;
mod game;
mod home;
mod login;
mod register;
use create_game::create_game;
use game::game;
use home::home;
use login::login;
use mongodb::Client;
use register::register;
use serde::Serialize;


fn _body<T>(body: T) -> String
where
    T: Serialize,
{
    serde_json::to_string(&body).unwrap()
}

pub fn ok<T>(body: T) -> Body
where
    T: Serialize,
{
    Body::from(ResponseBody::to_body(200, _body(body)))
}

pub fn create<T>(body: T) -> Body
where
    T: Serialize,
{
    Body::from(ResponseBody::to_body(201, _body(body)))
}

pub fn method_not_allowed() -> Body
{
    Body::from(ResponseBody::to_body(405, String::new()))
}

pub fn bad_request() -> Body
{
    Body::from(ResponseBody::to_body(400, String::new()))
}

pub fn error(error: crate::database::DatabaseError) -> Body
{
    let body = _body(&format!("{error:?}"));
    Body::from(ResponseBody::to_body(500, body))
}

pub fn not_found() -> Body
{
    Body::from(ResponseBody::to_body(404, String::new()))
}
trait CorsExt
{
    fn add_cors_headers(self) -> Self;
}

impl<T> CorsExt for Response<T>
{
    fn add_cors_headers(mut self) -> Self
    {
        let headers = self.headers_mut();
        headers.insert("Access-Control-Allow-Headers", "Content-Type".parse().unwrap());
        headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
        headers.insert("Access-Control-Allow-Methods", "GET, POST, PUT".parse().unwrap());
        self
    }
}

pub async fn get_body<T: serde::de::DeserializeOwned>(req: Request<Body>) -> Option<T>
{
    let body = body::to_bytes(req.into_body()).await.unwrap();
    let s = std::str::from_utf8(&body).ok()?;
    serde_json::from_str::<T>(s).ok()
}

fn log_req(req: &Request<Body>)
{
    use chrono::prelude::*;
    let method = req.method().to_string();
    let url = req.uri();

    let now = Utc::now().format("%d-%m-%y %H:%M:%S");
    println!("[{now}]\t{method}\t{url}");
}

async fn handle_request(req: Request<Body>, client: Client) -> Response<Body>
{
    if cfg!(debug_assertions)
    {
        log_req(&req);
    }
    match req.uri().path()
    {
        "/register" => register(req, client).await,
        "/login" => login(req, client).await,
        "/create-game" => create_game(req, client).await,
        "/home" => home(req, client).await,
        "/game" => game(req, client).await,
        _ => Response::new(not_found()),
    }
}

pub async fn handle(req: Request<Body>, client: Client) -> Result<Response<Body>, Infallible>
{
    Ok(match *req.method()
    {
        Method::OPTIONS => Response::new(Body::default()),
        _ => handle_request(req, client).await,
    }
    .add_cors_headers())
}
