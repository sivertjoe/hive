use mongodb::{
    bson::doc,
    error::Error,
    options::{ClientOptions, Credential},
    Client, Database,
};
use shared::{model::UserCredentials, Uuid};

use crate::user::User;


pub const LIVE: &str = "live";


// collections types
pub const USERS: &str = "users";
pub const CREATE_GAME: &str = "create-game";


#[derive(Debug)]
pub enum DatabaseError
{
    UserAlreadyExist,
    UserDontExist,
    DbError(mongodb::error::Error),
}
pub type DatabaseResult<T> = Result<T, DatabaseError>;


pub async fn connect() -> Result<Client, Error>
{
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    client_options.app_name = Some("My App".to_string());

    client_options.credential = Some(
        Credential::builder()
            .username(Some("root".to_string()))
            .password(Some("rootpassword".to_string()))
            .build(),
    );

    Client::with_options(client_options)
}

pub async fn login(db: Database, cred: UserCredentials) -> DatabaseResult<Uuid>
{
    let col = db.collection::<User>(USERS);

    // Check if user with same name exists
    let password_hash = hash(&cred.password);
    let filter = doc! { "name": cred.name.as_str(), "password_hash": password_hash.as_str() };
    match col.find_one(filter, None).await
    {
        Ok(Some(user)) => Ok(user.uuid),
        Ok(None) => Err(DatabaseError::UserDontExist),
        Err(e) => Err(DatabaseError::DbError(e)),
    }
}


pub async fn register_user(db: Database, cred: UserCredentials) -> DatabaseResult<Uuid>
{
    let col = db.collection::<User>(USERS);
    let user = User::from_cred(cred);

    // Check if user with same name exists
    let filter = doc! { "name": user.name.as_str() };
    match col.find_one(filter, None).await
    {
        Ok(Some(_)) => Err(DatabaseError::UserAlreadyExist),
        Ok(None) => match col.insert_one(&user, None).await
        {
            Ok(_) => Ok(user.uuid),
            Err(e) => Err(DatabaseError::DbError(e)),
        },
        Err(e) => Err(DatabaseError::DbError(e)),
    }
}

pub async fn find_user_by_uuid(db: Database, uuid: Uuid) -> DatabaseResult<User>
{
    let col = db.collection::<User>(USERS);

    let filter = doc! { "uuid": uuid };
    match col.find_one(filter, None).await
    {
        Ok(Some(user)) => Ok(user),
        Ok(None) => Err(DatabaseError::UserDontExist),
        Err(e) => Err(DatabaseError::DbError(e)),
    }
}

pub fn hash(word: &str) -> String
{
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(word);
    let result = hasher.finalize();
    format!("{result:x}")
}

pub fn uuid() -> String
{
    format!("{}", uuid::Uuid::new_v4())
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
    use mongodb::{Collection, Database};

    use super::*;
    use crate::user::User;


    struct Guard<T>
    {
        database:   Database,
        collection: Collection<T>,
    }

    impl<T> Drop for Guard<T>
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

    async fn get_collection<T>() -> Result<Guard<T>, DatabaseError>
    {
        let client = connect().await?;

        let name = format!("{}", uuid::Uuid::new_v4());
        let database = client.database(&name);
        let collection = database.collection::<T>(&name);

        Ok(Guard {
            database,
            collection,
        })
    }


    #[tokio::test(flavor = "multi_thread")]
    async fn test_can_register_and_find_user() -> Result<(), DatabaseError>
    {
        let guard = get_collection::<User>().await?;

        let cred = UserCredentials {
            name: "sivert".into(), password: "password".into()
        };

        let res = register_user(guard.database.clone(), cred).await;
        assert!(res.is_ok());

        let res = find_user_by_uuid(guard.database.clone(), res.unwrap()).await;
        assert!(res.is_ok());

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_register_user_errors() -> Result<(), DatabaseError>
    {
        let guard = get_collection::<User>().await?;

        let cred = UserCredentials {
            name: "sivert".into(), password: "password".into()
        };

        let res = find_user_by_uuid(guard.database.clone(), "totaly-a-uuid".to_string()).await;
        assert!(matches!(res, Err(DatabaseError::UserDontExist)));

        let res = register_user(guard.database.clone(), cred.clone()).await;
        assert!(res.is_ok());

        let res = register_user(guard.database.clone(), cred).await;
        assert!(matches!(res, Err(DatabaseError::UserAlreadyExist)));

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_not_registered_user_can_not_login() -> Result<(), DatabaseError>
    {
        let guard = get_collection::<User>().await?;
        let cred = UserCredentials {
            name: "sivert".into(), password: "password".into()
        };
        let res = login(guard.database.clone(), cred).await;
        assert!(matches!(res, Err(DatabaseError::UserDontExist)));

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_user_can_login() -> Result<(), DatabaseError>
    {
        let guard = get_collection::<User>().await?;

        let cred = UserCredentials {
            name: "sivert".into(), password: "password".into()
        };

        let reg_uuid = register_user(guard.database.clone(), cred.clone()).await?;

        let res = login(guard.database.clone(), cred).await;
        assert!(res.is_ok());
        let log_uuid = res.unwrap();

        assert_eq!(log_uuid, reg_uuid);

        Ok(())
    }
}
