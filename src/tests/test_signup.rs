#[cfg(test)]
mod tests {
    use crate::graphql_schema::users::mutation::signup::AddUser;
    use crate::password::PassWordHasher;
    use async_graphql::*;
    use diesel_async::pooled_connection::AsyncDieselConnectionManager;
    use diesel_async::pooled_connection::deadpool::Pool;
    use mockall::mock;
    use mockall::predicate::*;
    use std::env;
    use uuid::Uuid;

    

    // Mock the email service
    mock! {
        BrevoApi {
            fn send_verification_email(&self, email: String, code: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
        }
    }

    #[tokio::test]
    async fn test_signup() {
        dotenvy::dotenv().ok();

        // Setup the connection pool
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let config =
            AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
        let pool = Pool::builder(config).build().unwrap();

        let password_hasher = PassWordHasher::new();

        // Setup mocks
        let mut mock_brevo_api = MockBrevoApi::new();
        mock_brevo_api
            .expect_send_verification_email()
            .with(always(), always())
            .returning(|_, _| Ok(()));

        

        // Setup the context with mocked dependencies
        let schema = Schema::build(AddUser, AddUser, EmptySubscription)
            .data(password_hasher)
            .data(pool)
            .data(mock_brevo_api)
            .finish();

        let result = schema
            .execute(
                r#"
    mutation {
        signup(input: {
            firstName: "Test",
            lastName: "User",
            username: "testuser",
            userEmail: "test@example.com",
            passwordHash: "password",
            dateOfBirth: "2000-01-01"
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

        println!("{:#?}", result);

        // Assert the result
        assert!(result.is_ok());
    }
}
