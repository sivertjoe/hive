use std::convert::Infallible;

use hyper::{body, Body, Method, Request, Response};
use shared::model::http::*;

mod register;
use mongodb::Client;
use register::register;


pub fn create(body: String) -> Body
{
    Body::from(ResponseBody::to_body(201, body))
}

pub fn method_not_allowed() -> Body
{
    Body::from(ResponseBody::to_body(405, String::new()))
}

pub fn error(error: crate::database::DatabaseError) -> Body
{
    let body = serde_json::to_string(&format!("{error:?}")).unwrap();
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
        self
    }
}

pub async fn get_body<T: serde::de::DeserializeOwned>(req: Request<Body>) -> Option<T>
{
    let body = body::to_bytes(req.into_body()).await.unwrap();
    let s = std::str::from_utf8(&body).ok()?;
    serde_json::from_str::<T>(s).ok()
}

async fn handle_request(req: Request<Body>, client: Client) -> Response<Body>
{
    match req.uri().path()
    {
        "/register" => register(req, client).await,
        _ => Response::new(not_found()),
    }
}

pub async fn handle(req: Request<Body>, client: Client) -> Result<Response<Body>, Infallible>
{
    println!("Got request!");
    Ok(match *req.method()
    {
        Method::OPTIONS => Response::new(Body::default()),
        _ => handle_request(req, client).await,
    }
    .add_cors_headers())
}
