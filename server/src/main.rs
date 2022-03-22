mod user;

mod routing;
use routing::handle;
mod database;
use std::convert::Infallible;

use database::*;
use hyper::{
    service::{make_service_fn, service_fn},
    Server,
};

type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
pub async fn main() -> Result<(), Error>
{
    let client = connect().await?;
    /*client
    .database("live")
    .collection::<shared::model::Game>("games")
    .drop(None)
    .await
    .unwrap();*/

    let make_svc = make_service_fn(|_| {
        let client = client.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                let client = client.clone();
                async move { handle(req, client).await }
            }))
        }
    });

    let addr = ([0, 0, 0, 0], 5000).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
