#[cfg(test)]
mod tests {
    use crate::graphql_schema::users::mutation::login::{Login, LoginInput};
    use async_graphql::{EmptyMutation, EmptySubscription, Schema};
    use diesel_async::pooled_connection::deadpool::Pool;
    use diesel_async::pooled_connection::AsyncDieselConnectionManager;
    use mockall::mock;
    use mockall::predicate::eq;
    use std::env;
    use std::sync::Arc;

    // Mock the password hasher
    mock! {
        PassWordHasher {
            fn verify_password(&self, password: String, hash: String) -> bool;
        }
    }

    #[tokio::test]
    async fn test_login() {
        // Setup the mock password hasher
        let mut mock_hasher = MockPassWordHasher::new();
        mock_hasher
            .expect_verify_password()
            .with(
                eq("password".to_string()),
                eq("hashed_password".to_string()),
            )
            .returning(|_, _| true);

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let config =
            AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
        let pool = Pool::builder(config).build().unwrap();

        // Setup the GraphQL schema
        let schema = Schema::build(Login, EmptyMutation, EmptySubscription)
            .data(Arc::new(mock_hasher))
            .data(Arc::new(pool))
            .finish();

        // Create the login input
        let _login_input = LoginInput {
            user_email: "test@example.com".to_string(),
            password: "password".to_string(),
        };

        // Execute the login mutation
        let result = schema
            .execute(
                r#"
            mutation {
                login(input: {
                    userEmail: "test@example.com",
                    password: "password"
                }) {
                    id
                    firstName
                    lastName
                    username
                    userEmail
                    displayName
                    dateOfBirth
                }
            }
        "#,
            )
            .await;

        // Assert the result
        assert!(result.is_ok());
        // let data = result.data;
        // assert_eq!(data["login"], "User authenticated");
    }
}
