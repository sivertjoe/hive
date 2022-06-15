use std::convert::Infallible;

use hyper::{body, Body, Method, Request, Response};
use shared::model::http::*;

use crate::database::DatabaseError;


mod create_game;
mod game;
mod home;
mod login;
mod register;
use create_game::create_game;
use game::game;
use home::home;
use login::login;
use register::register;
use serde::Serialize;

use crate::State;

#[derive(Debug)]
pub enum HttpError
{
    Serialize,
    MethodNotAllowed,
    NotFound,
    Database(DatabaseError),
    Channel(Box<dyn std::error::Error>),
}

use serde::ser::Serializer;
impl Serialize for HttpError
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{:?}", self);
        serializer.collect_str(&s)
    }
}


pub enum HttpResult
{
    Ok(Box<dyn erased_serde::Serialize>),
    Create(Box<dyn erased_serde::Serialize>),
    Err(HttpError),
}

impl HttpResult
{
    pub fn new<F, T>(f: F, v: T) -> Self
    where
        F: Fn(Box<dyn erased_serde::Serialize>) -> Self,
        T: erased_serde::Serialize + 'static,
    {
        f(Box::new(v))
    }
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
        headers.insert("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE".parse().unwrap());
        self
    }
}

pub async fn get_body<T: serde::de::DeserializeOwned>(req: Request<Body>) -> Option<T>
{
    let body = body::to_bytes(req.into_body()).await.ok()?;
    let s = std::str::from_utf8(&body).ok()?;
    serde_json::from_str::<T>(s).ok()
}

fn log_req(req: &Request<Body>) -> String
{
    use chrono::prelude::*;
    let method = req.method().to_string();
    let url = req.uri();

    let now = Utc::now().format("%d-%m-%y %H:%M:%S");
    format!("[{now}]\t{method}\t{url}")
}

fn log_resp(resp: &HttpResult, time: std::time::Duration) -> String
{
    match resp
    {
        HttpResult::Ok(_) => format!("200 OK\t{}", time.as_secs_f32()),
        HttpResult::Create(_) => format!("201 Create\t{}", time.as_secs_f32()),
        HttpResult::Err(e) => format!("{} {:?}\t{}", error_code(e), e, time.as_secs_f32()),
    }
}

#[inline]
fn error_code(e: &HttpError) -> u32
{
    use HttpError::*;
    match e
    {
        Serialize => 400,
        NotFound => 404,
        MethodNotAllowed => 405,
        Database(_) | Channel(_) => 500,
    }
}

impl From<HttpResult> for Response<Body>
{
    fn from(http_res: HttpResult) -> Self
    {
        let f = |code: u32, t: &dyn erased_serde::Serialize| -> Response<Body> {
            Response::new(Body::from(ResponseBody::to_body(
                code,
                serde_json::to_string(&t).unwrap(),
            )))
        };

        match http_res
        {
            HttpResult::Ok(t) => f(200, &t),
            HttpResult::Create(t) => f(201, &t),
            HttpResult::Err(e) => f(error_code(&e), &e),
        }
    }
}


async fn handle_request(req: Request<Body>, state: State) -> Response<Body>
{
    if let Some(path) = req.uri().path().strip_prefix("/api/")
    {
        let mut info = log_req(&req);
        info.push(' ');

        let now = std::time::Instant::now();
        let resp = match path
        {
            "register" => register(req, state).await,
            "login" => login(req, state).await,
            "game" => game(req, state).await,
            "create-game" => create_game(req, state).await,
            "home" => home(req, state).await,

            _ => HttpResult::Err(HttpError::NotFound),
        };

        info.push_str(&log_resp(&resp, now.elapsed()));
        println!("{}", info);
        resp
    }
    else
    {
        HttpResult::Err(HttpError::NotFound)
    }
    .into()
}

pub async fn handle(req: Request<Body>, state: State) -> Result<Response<Body>, Infallible>
{
    Ok(match *req.method()
    {
        Method::OPTIONS => Response::new(Body::default()),
        _ => handle_request(req, state).await,
    }
    .add_cors_headers())
}
