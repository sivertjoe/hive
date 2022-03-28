mod model;
mod routing;
mod websocket;
use routing::handle;
mod database;
use std::convert::Infallible;

use database::*;
use hyper::{
    service::{make_service_fn, service_fn},
    Server,
};
use websocket::*;


type SError = Box<dyn std::error::Error + Send + Sync>;


#[tokio::main]
pub async fn main() -> Result<(), SError>
{
    let client = connect().await?;
    if false
    {
        client.database("live").drop(None).await.unwrap();
        println!("DROPPING");
    }

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


    // Use `tx` to communicate when a move has occured!
    let (tx, rx) = tokio::sync::mpsc::channel(10); // 10 good??


    tokio::spawn(spawn_web_socket_server(rx));

    println!("Listening on http://{}", addr);
    server.await?;

    Ok(())
}
