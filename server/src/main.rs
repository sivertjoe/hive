mod model;
mod routing;
mod websocket;
use routing::handle;
mod database;
use std::convert::Infallible;

use hyper::{
    service::{make_service_fn, service_fn},
    Server,
};
use websocket::*;


type SError = Box<dyn std::error::Error + Send + Sync>;

use tokio::sync::mpsc;
use websocket::Message;

#[derive(Clone)]
pub struct State
{
    pub client: mongodb::Client,
    pub tx:     mpsc::Sender<Message>,
}

impl State
{
    pub fn db(&self) -> mongodb::Database
    {
        self.client.database(database::LIVE)
    }
}


async fn spawn_http_server(state: State) -> Result<(), SError>
{
    let make_svc = make_service_fn(|_| {
        let state = state.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                let state = state.clone();
                handle(req, state)
            }))
        }
    });

    let addr = ([0, 0, 0, 0], 5000).into();
    let server = Server::bind(&addr).serve(make_svc);
    println!("Listening on http://{}", addr);
    server.await?;
    Ok(())
}


#[tokio::main]
pub async fn main() -> Result<(), SError>
{
    let client = database::connect().await?;
    let (tx, rx) = tokio::sync::mpsc::channel(10); // 10 good??

    let state = State {
        client,
        tx,
    };


    tokio::spawn(spawn_web_socket_server(rx));
    let _ = spawn_http_server(state).await;

    Ok(())
}
