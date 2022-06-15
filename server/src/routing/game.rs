use hyper::{Body, Method, Request};
use mongodb::bson::oid::ObjectId;
use shared::model::Move;

use super::{get_body, HttpError, HttpResult};
use crate::{
    database::{
        complete_game, get_active_games, get_game_by_id, get_old_games, get_users_games, play_move,
        DatabaseError::GameNotComplete,
    },
    State,
};

/*
 * Function for games. The http methods are:
 *     GET   : get game/games
 *     POST  : play a move on a game
 *     DElETE: delete a finished game
 */
pub async fn game(req: Request<Body>, state: State) -> HttpResult
{
    match *req.method()
    {
        Method::GET => get(req, state).await,
        Method::POST => post(req, state).await,
        Method::DELETE => delete(req, state).await,
        _ => HttpResult::Err(HttpError::MethodNotAllowed),
    }
}

/*
 * Get game/games based on the query in the URL, See Query::from_str for more
 * info
 */
async fn get(req: Request<Body>, state: State) -> HttpResult
{
    match req.uri().query().and_then(|uri| Query::from_str(uri).ok())
    {
        Some(q) => match q
        {
            Query::All => match get_active_games(state.db()).await
            {
                Ok(res) => HttpResult::new(HttpResult::Ok, res),
                Err(e) => HttpResult::Err(HttpError::Database(e)),
            },

            Query::Old => match get_old_games(state.db()).await
            {
                Ok(res) => HttpResult::new(HttpResult::Ok, res),
                Err(e) => HttpResult::Err(HttpError::Database(e)),
            },

            Query::Id(object_id) => match get_game_by_id(state.db(), object_id).await
            {
                Ok(res) => HttpResult::new(HttpResult::Ok, res),
                Err(e) => HttpResult::Err(HttpError::Database(e)),
            },
            Query::User(object_id) => match get_users_games(state.db(), object_id).await
            {
                Ok(res) => HttpResult::new(HttpResult::Ok, res),
                Err(e) => HttpResult::Err(HttpError::Database(e)),
            },
        },
        _ => HttpResult::Err(HttpError::Serialize),
    }
}


/*
 * Function for handling moves.
 * Expects a Move struct in the request body.
 * Additionally sends the move to the websocket server to send to the
 * spectators.
 */
async fn post(req: Request<Body>, state: State) -> HttpResult
{
    match get_body::<Move>(req).await
    {
        Some(r#move) => match play_move(state.db(), r#move.clone()).await
        {
            Ok(()) =>
            {
                let msg = crate::websocket::Message {
                    r#move,
                };
                match state.tx.send(msg).await
                {
                    Ok(_) => HttpResult::new(HttpResult::Ok, ()),
                    Err(e) => HttpResult::Err(HttpError::Channel(Box::new(e))),
                }
            },
            Err(e) => HttpResult::Err(HttpError::Database(e)),
        },
        _ => HttpResult::Err(HttpError::Serialize),
    }
}

/*
 * Function for completing a game.
 * Expects the ObjectId of the game being deleted in the Body.
 */
async fn delete(req: Request<Body>, state: State) -> HttpResult
{
    match get_body(req).await
    {
        Some(id) => match get_game_by_id(state.db(), id).await
        {
            Ok(game) =>
            {
                if game.board.is_complete()
                {
                    complete_game(state.db(), id).await.unwrap();
                    HttpResult::new(HttpResult::Ok, ())
                }
                else
                {
                    HttpResult::Err(HttpError::Database(GameNotComplete))
                }
            },
            Err(e) => HttpResult::Err(HttpError::Database(e)),
        },
        _ => HttpResult::Err(HttpError::Serialize),
    }
}

enum Query
{
    All,
    Old,
    Id(ObjectId),
    User(ObjectId),
}


/*
 * Every URL starts with q=
 *     user(<id>) gets all the users (ongoing) games
 *     all        fetches all games
 *     old        fetches all COMPLETE games
 *     <id>       fetches the game with the corresponding id
 */
use std::str::FromStr;
impl FromStr for Query
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        // HAS to start with q= atleast
        if s.len() <= 2
        {
            return Err(());
        }

        if s.starts_with("q=user")
        {
            // get the string s within "q=user(s)""
            let id = &s[7..s.len() - 1];
            match ObjectId::parse_str(id)
            {
                Ok(id) => Ok(Query::User(id)),
                _ => Err(()),
            }
        }
        else
        {
            match &s[2..]
            {
                "all" => Ok(Query::All),
                "old" => Ok(Query::Old),
                s => match ObjectId::parse_str(s)
                {
                    Ok(id) => Ok(Query::Id(id)),
                    _ => Err(()),
                },
            }
        }
    }
}
