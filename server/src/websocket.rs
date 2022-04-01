use tokio::{
    net::{TcpListener, TcpStream},
    select,
    sync::mpsc,
};
use tokio_tungstenite::{accept_hdr_async, WebSocketStream};
use tungstenite::{handshake::server::Request, Result};


pub struct Message
{
    pub r#move: Move,
}

use std::collections::HashMap;

use mongodb::bson::oid::ObjectId;
use shared::model::game::Move;

#[derive(Default)]
struct State
{
    map: HashMap<ObjectId, Vec<mpsc::Sender<Move>>>,
}

impl State
{
    async fn send_updates(&mut self, r#move: Move)
    {
        let game_id = r#move.game_id.clone();
        if let Some(senders) = self.map.remove(&game_id)
        {
            use futures::stream::{self, StreamExt};
            let new = stream::iter(senders)
                .filter_map(|tx| {
                    let mv = r#move.clone();
                    async move { tx.send(mv).await.ok().map(|_| tx) }
                })
                .collect::<Vec<mpsc::Sender<Move>>>()
                .await;

            self.map.insert(game_id, new);
        }
    }

    fn add_sender(&mut self, id: ObjectId, sender: mpsc::Sender<Move>)
    {
        self.map.entry(id).or_default().push(sender);
    }
}

async fn get_websocket_and_uri(
    stream: TcpStream,
) -> Result<(WebSocketStream<TcpStream>, hyper::Uri), tungstenite::Error>
{
    let mut uri = None;
    let ws_stream = accept_hdr_async(stream, |req: &Request, res| {
        uri = Some(req.uri().clone());
        Ok(res)
    })
    .await?;
    let uri = uri.unwrap();
    Ok((ws_stream, uri))
}

pub async fn spawn_web_socket_server(mut rx: mpsc::Receiver<Message>)
{
    let addr = "0.0.0.0:5001";
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    println!("spawning websockert addr at {}", addr);

    let mut state = State::default();

    loop
    {
        select! {
           msg = rx.recv() => {
               if let Some(msg) = msg {
                state.send_updates(msg.r#move).await;
               }
            },

            res = listener.accept()=> {
                if res.is_err() { return; }
                let (stream, _) = res.unwrap();

                let res = get_websocket_and_uri(stream).await;
                if res.is_err() { return; }
                let (ws, uri) = res.unwrap();

                let game_id = get_game_id(uri);

                let (tx, rx) = mpsc::channel(10); // 10?
                state.add_sender(game_id, tx);

                tokio::spawn(handle_connection(ws, rx));
            }
        };
    }
}

async fn handle_connection(mut ws: WebSocketStream<TcpStream>, mut rx: mpsc::Receiver<Move>)
{
    use futures::{stream::StreamExt, SinkExt};
    println!("ENTER");

    use tungstenite::{Error::ConnectionClosed, Message::Close};

    loop
    {
        select! {
            res = ws.next() =>
            {
                if let Some(msg) = res
                {
                    match msg
                    {
                        Ok(Close(_)) | Err(ConnectionClosed) => {
                            break;
                        }
                        e => panic!("{:?}", e),
                    }
                }

            }

            msg = rx.recv() => {
                if let Some(r#move) = msg
                {
                    let text = serde_json::to_string(&r#move).unwrap();
                    let msg = tungstenite::Message::Text(text);
                    if let Err(_) = ws.send(msg).await
                    {
                        break;
                    }
                }
            }
        }
    }
    println!("DROPEED");
}


fn get_game_id(uri: hyper::Uri) -> ObjectId
{
    let q = uri.query().unwrap();
    q.strip_prefix("id=").unwrap().parse::<ObjectId>().unwrap()
}
