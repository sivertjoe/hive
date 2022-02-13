use mongodb::{
    error::Error,
    options::{ClientOptions, Credential},
    Client,
};

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


#[cfg(test)]
mod test
{
    use mongodb::{error::Error, Collection, Database};

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

    async fn get_collection<T>() -> Result<Guard<T>, Error>
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

    use futures::StreamExt;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_can_insert_and_get_user() -> Result<(), Error>
    {
        let guard = get_collection::<User>().await?;

        let users = vec![
            User::new(String::from("Sofie"), String::from("password")),
            User::new(String::from("Sivert"), String::from("password")),
        ];

        guard.collection.insert_many(users, None).await?;

        let users: Vec<User> =
            guard.collection.find(None, None).await?.map(|res| res.unwrap()).collect().await;

        assert!(users.len() == 2);


        Ok(())
    }
}
