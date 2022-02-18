use std::convert::Infallible;

use hyper::{body, Body, Method, Request, Response};
use shared::model::UserCredentials;


trait CorsExt
{
    fn add_cors_headers(self) -> Self;
}

impl<T> CorsExt for Response<T>
{
    fn add_cors_headers(mut self) -> Self
    {
        let mut headers = self.headers_mut();
        headers.insert("Access-Control-Allow-Headers", "Content-Type".parse().unwrap());
        headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
        self
    }
}

async fn get_body<T: serde::de::DeserializeOwned>(req: Request<Body>) -> Option<T>
{
    let body = body::to_bytes(req.into_body()).await.unwrap();
    let s = std::str::from_utf8(&body).ok()?;
    dbg!(s);
    serde_json::from_str::<T>(s).ok()
}

async fn register(req: Request<Body>) -> Response<Body>
{
    match *req.method()
    {
        Method::POST =>
        {
            let cred = get_body::<UserCredentials>(req).await.unwrap();
            println!("{cred:?}");
            Response::new(Body::from("SUCCESS"))
        },
        _ => Response::new(Body::from("ERROR")),
    }
}

async fn handle_request(req: Request<Body>) -> Response<Body>
{
    match req.uri().path()
    {
        "/register" => register(req).await,
        _ => Response::new(Body::from("idk man")),
    }
}

pub async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible>
{
    println!("Got request!");
    Ok(match *req.method()
    {
        Method::OPTIONS => Response::new(Body::default()),
        _ => handle_request(req).await,
    }
    .add_cors_headers())
}
