use const_format::formatcp;
use futures::stream::StreamExt;
use mongodb::{
    bson::{self, doc, oid::ObjectId},
    error::Error,
    options::{ClientOptions, Credential},
    Client, Database,
};
use shared::model::*;

use crate::model::{game::Game, user::User};


pub const LIVE: &str = "live";
pub const MAX_CREATE_GAME: u32 = 25;


pub const PROTOCOL: &str = "mongodb";

pub const URL: &str = if cfg!(debug_assertions) { "0.0.0.0" } else { "db" };
pub const PORT: usize = 27017;
pub const DB_PATH: &str = formatcp!("{PROTOCOL}://{URL}:{PORT}");

// collections types
pub const USERS: &str = "users";
pub const GAMES: &str = "games";


#[derive(Debug)]
pub enum DatabaseError
{
    UserAlreadyExist,
    UserDontExist,
    DbError(mongodb::error::Error),
    NoDocumentFound,
    TooManyGames,
    GameNotComplete,
}


pub type DatabaseResult<T> = Result<T, DatabaseError>;


pub async fn connect() -> Result<Client, Error>
{
    let mut client_options = ClientOptions::parse(DB_PATH).await?;
    client_options.app_name = Some("My App".to_string());

    client_options.credential = Some(
        Credential::builder()
            .username(Some("root".to_string()))
            .password(Some("rootpassword".to_string()))
            .build(),
    );

    Client::with_options(client_options)
}

pub async fn login(db: Database, cred: UserCredentials) -> DatabaseResult<ObjectId>
{
    let col = db.collection::<User>(USERS);

    // Check if user with same name exists
    let password_hash = hash(&cred.password);
    let filter = doc! { "name": cred.name.as_str(), "password_hash": password_hash.as_str() };
    match col.find_one(filter, None).await?
    {
        Some(user) => Ok(user._id),
        None => Err(DatabaseError::UserDontExist),
    }
}


pub async fn register_user(db: Database, cred: UserCredentials) -> DatabaseResult<ObjectId>
{
    let col = db.collection::<User>(USERS);
    let user = User::from_cred(cred);

    // Check if user with same name exists
    let filter = doc! { "name": user.name.as_str() };
    match col.find_one(filter, None).await?
    {
        Some(_) => Err(DatabaseError::UserAlreadyExist),
        None => match col.insert_one(&user, None).await
        {
            Ok(_) => Ok(user._id),
            Err(e) => Err(DatabaseError::DbError(e)),
        },
    }
}

#[allow(dead_code)]
pub async fn find_user_by_id(db: Database, id: ObjectId) -> DatabaseResult<User>
{
    let col = db.collection::<User>(USERS);

    let filter = doc! { "_id": id };
    match col.find_one(filter, None).await?
    {
        Some(user) => Ok(user),
        None => Err(DatabaseError::UserDontExist),
    }
}
async fn count_create_game(db: Database, id: &ObjectId) -> DatabaseResult<u32>
{
    let col = db.collection::<User>(USERS);
    let sum = "sum";
    let r#match = doc! { "$match": { "_id": &id } };
    let crit = doc! { "$project": { sum: { "$size": "$create_games" } } };

    match col.aggregate([r#match, crit], None).await?.next().await
    {
        Some(doc) =>
        {
            let num = doc?.get(sum).unwrap().as_i32().unwrap() as u32;
            Ok(num)
        },
        None => Err(DatabaseError::NoDocumentFound),
    }
}

pub async fn create_game(db: Database, user_id: ObjectId) -> DatabaseResult<ObjectId>
{
    let col = db.collection::<User>(USERS);

    if count_create_game(db, &user_id).await? > MAX_CREATE_GAME
    {
        return Err(DatabaseError::TooManyGames);
    }

    let user = doc! { "_id": &user_id };

    let id = ObjectId::new();
    let update = doc! { "$push": { "create_games": id.clone() } };

    match col.update_one(user, update, None).await
    {
        Ok(_) => Ok(id),
        Err(e) => Err(e.into()),
    }
}

async fn remove_user_create_game(
    db: Database,
    creator: &ObjectId,
    game: &ObjectId,
) -> DatabaseResult<()>
{
    let col = db.collection::<User>(USERS);

    col.update_one(doc! { "_id": creator }, doc! { "$pull": { "create_games": game } }, None)
        .await?;
    Ok(())
}

async fn add_game_id_to_users(
    db: Database,
    id: &ObjectId,
    user1: &ObjectId,
    user2: &ObjectId,
) -> DatabaseResult<()>
{
    let col = db.collection::<User>(USERS);

    let user = doc! {"$or": [ {"_id": &user1}, {"_id": &user2} ]};
    let update = doc! { "$push": { "active_games": id } };

    match col.update_many(user, update, None).await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

pub async fn accept_game(db: Database, form: CreateGameFormResponse) -> DatabaseResult<AcceptGame>
{
    let CreateGameFormResponse {
        user,
        game,
        creator,
    } = form;

    remove_user_create_game(db.clone(), &creator, &game).await?;

    let games = db.collection::<Game>(GAMES);


    // @NOTE: Take a look at this in the future haha Xdd
    use getrandom::getrandom;
    let mut byte = [0_u8];
    getrandom(&mut byte).expect("random");

    let players = if byte[0] >= 128 { [creator, user] } else { [user, creator] };

    let (u1, u2) = tokio::join!(
        find_user_by_id(db.clone(), players[0].clone()),
        find_user_by_id(db.clone(), players[1].clone())
    );
    let users = [u1?, u2?];

    let id = games
        .insert_one(Game::new(users), None)
        .await?
        .inserted_id
        .as_object_id()
        .unwrap();

    add_game_id_to_users(db.clone(), &id, &creator, &user).await?;

    let accept = AcceptGame {
        object_id: id,
        game,
    };

    Ok(accept)
}

pub async fn home(db: Database, id: ObjectId) -> DatabaseResult<Vec<CreateGameChallengeBundle>>
{
    let col = db.collection::<User>(USERS);
    let filter = doc! { "_id": { "$ne": id } };

    Ok(col
        .find(filter, None)
        .await?
        .map(|res| {
            let res = res.unwrap();

            CreateGameChallengeBundle {
                name:       res.name,
                games:      res.create_games,
                creator_id: res._id,
            }
        })
        .collect::<Vec<_>>()
        .await)
}

//TODO: Whats the point of the uuid? just use the _id
pub async fn get_active_games(db: Database) -> DatabaseResult<Vec<OnGoingGame>>
{
    let col = db.collection::<Game>(GAMES);


    Ok(col
        .aggregate(
            [
                doc! {
                    "$match": {
                        "complete": false
                    }
                },
                doc! {
                    "$project": {
                        "players": "$players.name",
                        "game_object_id": "$_id",
                    }
                },
            ],
            None,
        )
        .await?
        .map(|doc| {
            let doc = doc.unwrap();
            let g: OnGoingGame = bson::from_document(doc).unwrap();
            g
        })
        .collect()
        .await)
}


pub async fn get_game_by_id(db: Database, id: ObjectId) -> DatabaseResult<GameResource>
{
    let col = db.collection::<Game>(GAMES);
    col.aggregate(
        [
            doc! {
                "$match": { "_id": &id }
            },
            doc! {
                "$project": {
                    "players": "$players.name",
                    "board": "$board",
                    "complete": "$complete",
                }
            },
        ],
        None,
    )
    .await?
    .map(|doc| {
        let doc = doc.unwrap();
        let g: GameResource = bson::from_document(doc).unwrap();
        g
    })
    .next()
    .await
    .ok_or(DatabaseError::NoDocumentFound)
}

pub async fn play_move(db: Database, r#move: Move) -> DatabaseResult<()>
{
    let col = db.collection::<Game>(GAMES);

    let query = doc! {
        "_id": r#move.game_id,
        "players._id": &r#move.player_id,
    };

    let mut game =
        col.find_one(query.clone(), None).await?.ok_or(DatabaseError::NoDocumentFound)?;
    game.board.play_move(r#move);

    col.replace_one(query, game, None).await.map(|_| ()).map_err(|e| e.into())
}

pub async fn complete_game(db: Database, game_id: ObjectId) -> DatabaseResult<()>
{
    let col = db.collection::<Game>(GAMES);
    let filter = doc! { "_id": game_id };
    let update = doc! { "$set": { "complete": true } };

    col.find_one_and_update(filter, update, None)
        .await
        .map(|_| ())
        .map_err(|e| e.into())
}


pub fn hash(word: &str) -> String
{
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(word);
    let result = hasher.finalize();
    format!("{result:x}")
}

impl From<mongodb::error::Error> for DatabaseError
{
    fn from(error: mongodb::error::Error) -> Self
    {
        DatabaseError::DbError(error)
    }
}

#[cfg(test)]
mod test
{
    use mongodb::Database;

    use super::*;


    struct Guard
    {
        database: Database,
    }

    impl Guard
    {
        fn db(&self) -> Database
        {
            self.database.clone()
        }
    }

    impl Drop for Guard
    {
        fn drop(&mut self)
        {
            use tokio::{runtime::Handle, task};

            task::block_in_place(move || {
                Handle::current().block_on(async move {
                    self.database.drop(None).await.unwrap();
                });
            });
        }
    }

    async fn get_guard() -> Result<Guard, DatabaseError>
    {
        let client = connect().await?;

        let name = format!("{}", uuid::Uuid::new_v4());
        let database = client.database(&name);

        Ok(Guard {
            database,
        })
    }

    async fn reg(guard: &Guard, name: String) -> DatabaseResult<ObjectId>
    {
        let cred = UserCredentials {
            name,
            password: "password".into(),
        };
        register_user(guard.db(), cred).await
    }

    async fn create_users_and_game(guard: &Guard)
        -> DatabaseResult<(ObjectId, ObjectId, ObjectId)>
    {
        let u1 = reg(guard, "sivert".into()).await?;
        let u2 = reg(guard, "sofie".into()).await?;

        create_game(guard.db(), u1.clone()).await?;

        let games = home(guard.db(), u2.clone()).await?;
        let game = games[0].games[0].clone();

        let form = CreateGameFormResponse {
            creator: u1.clone(),
            user: u2.clone(),
            game,
        };

        let game = accept_game(guard.db(), form).await?.object_id;

        Ok((u1, u2, game))
    }


    #[tokio::test(flavor = "multi_thread")]
    async fn test_can_register_and_find_user() -> Result<(), DatabaseError>
    {
        let guard = get_guard().await?;

        let res = reg(&guard, "sivert".into()).await;
        assert!(res.is_ok());

        let res = find_user_by_id(guard.db(), res.unwrap()).await;
        assert!(res.is_ok());

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_register_user_errors() -> Result<(), DatabaseError>
    {
        let guard = get_guard().await?;

        let cred = UserCredentials {
            name: "sivert".into(), password: "password".into()
        };

        let res = find_user_by_id(guard.db(), ObjectId::new()).await;
        assert!(matches!(res, Err(DatabaseError::UserDontExist)));

        let res = register_user(guard.db(), cred.clone()).await;
        assert!(res.is_ok());

        let res = register_user(guard.db(), cred).await;
        assert!(matches!(res, Err(DatabaseError::UserAlreadyExist)));

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_not_registered_user_can_not_login() -> Result<(), DatabaseError>
    {
        let guard = get_guard().await?;
        let cred = UserCredentials {
            name: "sivert".into(), password: "password".into()
        };
        let res = login(guard.db(), cred).await;
        assert!(matches!(res, Err(DatabaseError::UserDontExist)));

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_user_can_login() -> Result<(), DatabaseError>
    {
        let guard = get_guard().await?;

        let cred = UserCredentials {
            name: "sivert".into(), password: "password".into()
        };

        let reg_uuid = register_user(guard.db(), cred.clone()).await?;

        let res = login(guard.db(), cred).await;
        assert!(res.is_ok());
        let log_uuid = res.unwrap();

        assert_eq!(log_uuid, reg_uuid);

        Ok(())
    }


    #[tokio::test(flavor = "multi_thread")]
    async fn test_user_can_create_game() -> Result<(), DatabaseError>
    {
        let guard = get_guard().await?;
        let creator = reg(&guard, "sivert".into()).await?;

        assert!(create_game(guard.db(), creator).await.is_ok());
        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_get_home_content() -> Result<(), DatabaseError>
    {
        let guard = get_guard().await?;
        let creator = reg(&guard, "sivert".into()).await?;

        assert!(create_game(guard.db(), creator.clone()).await.is_ok());
        assert!(create_game(guard.db(), creator.clone()).await.is_ok());

        assert_eq!(home(guard.db(), creator).await?.len(), 0);

        let new = reg(&guard, "sofie".into()).await?;
        let games = home(guard.db(), new).await?;
        assert_eq!(games[0].games.len(), 2);

        Ok(())
    }


    #[tokio::test(flavor = "multi_thread")]
    async fn test_count_create_game() -> Result<(), DatabaseError>
    {
        let guard = get_guard().await?;

        // Create user#1 and create one game
        let id = reg(&guard, "sofie".into()).await?;
        assert!(create_game(guard.db(), id.clone()).await.is_ok());


        // Create user#2 and `n` games
        let id = reg(&guard, "sivert".into()).await?;
        let n: u32 = 20;

        for _ in 0..n
        {
            assert!(create_game(guard.db(), id.clone()).await.is_ok());
        }

        assert_eq!(count_create_game(guard.db(), &id).await?, n);

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_remove_create_game() -> Result<(), DatabaseError>
    {
        let guard = get_guard().await?;
        let creator = reg(&guard, "sivert".into()).await?;

        let create_id = create_game(guard.db(), creator.clone()).await?;

        assert_eq!(find_user_by_id(guard.db(), creator.clone()).await?.create_games.len(), 1);
        assert!(remove_user_create_game(guard.db(), &creator, &create_id).await.is_ok());

        assert_eq!(find_user_by_id(guard.db(), creator.clone()).await?.create_games.len(), 0);


        Ok(())
    }


    #[tokio::test(flavor = "multi_thread")]
    async fn test_can_accept_game() -> Result<(), DatabaseError>
    {
        let guard = get_guard().await?;

        let creator = reg(&guard, "sivert".into()).await?;
        let create_id = create_game(guard.db(), creator.clone()).await?;

        let id = reg(&guard, "sofie".into()).await?;

        let form = CreateGameFormResponse {
            creator: creator.clone(),
            user:    id.clone(),
            game:    create_id,
        };

        assert!(accept_game(guard.db(), form).await.is_ok());

        let user = find_user_by_id(guard.db(), creator).await?;
        assert_eq!(user.create_games.len(), 0);
        assert_eq!(user.active_games.len(), 1);

        let user = find_user_by_id(guard.db(), id).await?;
        assert_eq!(user.active_games.len(), 1);

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_can_get_active_games() -> Result<(), DatabaseError>
    {
        let guard = get_guard().await?;

        let creator = reg(&guard, "sivert".into()).await?;
        let create_id = create_game(guard.db(), creator.clone()).await?;

        let uuid = reg(&guard, "sofie".into()).await?;

        let form = CreateGameFormResponse {
            creator: creator.clone(),
            user:    uuid.clone(),
            game:    create_id,
        };

        assert!(accept_game(guard.db(), form).await.is_ok());

        let vec = get_active_games(guard.db()).await?;
        assert_eq!(vec.len(), 1);

        Ok(())
    }


    #[tokio::test(flavor = "multi_thread")]
    async fn test_cannot_get_non_existing_game() -> Result<(), DatabaseError>
    {
        let guard = get_guard().await?;

        let res = get_game_by_id(guard.db(), ObjectId::new()).await;
        assert!(matches!(res, Err(DatabaseError::NoDocumentFound)));
        Ok(())
    }


    #[tokio::test(flavor = "multi_thread")]
    async fn test_can_get_game() -> Result<(), DatabaseError>
    {
        let guard = get_guard().await?;
        let (_, _, game_id) = create_users_and_game(&guard).await?;

        assert!(get_game_by_id(guard.db(), game_id).await.is_ok());

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_can_play_move() -> Result<(), DatabaseError>
    {
        let guard = get_guard().await?;
        let (u1, _, game_id) = create_users_and_game(&guard).await?;

        let game = get_game_by_id(guard.db(), game_id).await?;

        let mov = Move {
            player_id: u1,
            game_id:   game._id,
            sq:        (0, 0, 0),
            piece:     Piece::new(BoardPiece::Ant, Color::White),
            old_sq:    None,
        };

        assert!(play_move(guard.db(), mov).await.is_ok());

        let game = get_game_by_id(guard.db(), game_id).await?;
        assert_eq!(game.board.len(), 1);
        assert_eq!(game.board.turns, 1);

        Ok(())
    }
}
