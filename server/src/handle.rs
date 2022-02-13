use std::convert::Infallible;

use hyper::{Body, Method, Request, Response};

pub async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible>
{
    match *req.method()
    {
        Method::GET => Ok(Response::new(Body::from("Get request!"))),
        _ => Ok(Response::new(Body::from("Something else"))),
    }
}
