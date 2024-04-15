#[cfg(test)]
mod tests {
    use crate::graphql_schema::users::mutation::login::Login;
    use crate::password::PassWordHasher;
    use async_graphql::{EmptySubscription, Schema};
    use diesel_async::pooled_connection::deadpool::Pool;
    use diesel_async::pooled_connection::AsyncDieselConnectionManager;
    use std::env;

    #[tokio::test]
    async fn test_login() {
        dotenvy::dotenv().ok();
        // Setup the mock password hasher
        let password_hasher = PassWordHasher::new();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let config =
            AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
        let pool = Pool::builder(config).build().unwrap();

        // Setup the GraphQL schema
        let schema = Schema::build(Login, Login, EmptySubscription)
            .data(password_hasher)
            .data(pool)
            .finish();

        // Execute the login mutation
        let result = schema
            .execute(
                r#"
            mutation {
                login(input: {
                    userEmail: "test@example.com",
                    password: "password"
                }) 
            }
        "#,
            )
            .await;

        println!("{:#?}", result);

        // Assert the result
        assert!(result.is_ok());
    }
}
