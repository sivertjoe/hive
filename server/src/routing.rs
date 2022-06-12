use std::{convert::Infallible, future::Future};

use hyper::{body, Body, Method, Request, Response};
use shared::model::http::*;

use crate::database::{self, DatabaseError};


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


pub enum HttpResult<T: Serialize>
{
    Ok(T),
    Create(T),
    Err(HttpError),
}

/*
 * This badboy is mostly made for fun 😎. Do not do this 🤠.
 */
pub async fn data<F, Fut, B, S, Res, Fin>(
    func: F,
    req: Request<Body>,
    db: mongodb::Database,
    s: S,
) -> HttpResult<Fin>
where
    F: Fn(mongodb::Database, B) -> Fut,
    Fut: Future<Output = database::DatabaseResult<Res>>,
    S: Fn(Res) -> HttpResult<Fin>,
    Res: Serialize,
    Fin: Serialize,
    B: serde::de::DeserializeOwned,
{
    match get_body::<B>(req).await
    {
        Some(body) => match func(db, body).await
        {
            Ok(t) => s(t),
            Err(e) => HttpResult::Err(HttpError::Database(e)),
        },
        None => HttpResult::Err(HttpError::Serialize),
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
    let body = body::to_bytes(req.into_body()).await.unwrap();
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

fn log_resp<T: Serialize>(resp: &HttpResult<T>, time: std::time::Duration) -> String
{
    match resp
    {
        HttpResult::Ok(_) => format!("200 OK\t{}", time.as_secs_f32()),
        HttpResult::Create(_) => format!("201 Create\t{}", time.as_secs_f32()),
        HttpResult::Err(e) => format!("{} {:?}\t{}", error_code(&e), e, time.as_secs_f32()),
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

impl<T> Into<Response<Body>> for HttpResult<T>
where
    T: Serialize,
{
    fn into(self) -> Response<Body>
    {
        fn f<T>(code: u32, t: T) -> Response<Body>
        where
            T: serde::Serialize,
        {
            Response::new(Body::from(ResponseBody::to_body(
                code,
                serde_json::to_string(&t).unwrap(),
            )))
        }

        match self
        {
            HttpResult::Ok(t) => f(200, t),
            HttpResult::Create(t) => f(201, t),
            HttpResult::Err(e) => f(error_code(&e), e),
        }
    }
}

fn handle_response<T>(
    resp: HttpResult<T>,
    info: &mut String,
    now: std::time::Instant,
) -> Response<Body>
where
    T: Serialize,
{
    info.push_str(&log_resp(&resp, now.elapsed()));
    resp.into()
}

async fn handle_request(req: Request<Body>, state: State) -> Response<Body>
{
    if cfg!(debug_assertions)
    {
        log_req(&req);
    }

    if let Some(path) = req.uri().path().strip_prefix("/api/")
    {
        let mut info = log_req(&req);
        info.push(' ');

        let now = std::time::Instant::now();
        let res = match path
        {
            "register" => handle_response(register(req, state).await, &mut info, now),
            "login" => handle_response(login(req, state).await, &mut info, now),
            "game" => handle_response(game(req, state).await, &mut info, now),
            "create-game" => handle_response(create_game(req, state).await, &mut info, now),
            "home" => handle_response(home(req, state).await, &mut info, now),

            _ => handle_response(HttpResult::<()>::Err(HttpError::NotFound), &mut info, now),
        };

        println!("{}", info);
        res
    }
    else
    {
        HttpResult::<()>::Err(HttpError::NotFound).into()
    }
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
