mod user;

mod handle;
use handle::handle;
mod database;
use std::{convert::Infallible, io, pin::Pin};

use database::*;
use hyper::{
    service::{make_service_fn, service_fn},
    Server,
};


type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
pub async fn main() -> Result<(), Error>
{
    // let _client = connect().await?;
    let make_svc = make_service_fn(move |_| async move {
        Ok::<_, Infallible>(service_fn(move |req| async move { handle(req).await }))
    });

    let addr = ([0, 0, 0, 0], 5000).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;



    Ok(())
}
