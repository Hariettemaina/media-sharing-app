#[cfg(test)]
mod tests {
    use crate::graphql_schema::users::mutation::signup::AddUser;
    use crate::graphql_schema::users::mutation::signup::UserInput;
    use async_graphql::*;
    use diesel_async::pooled_connection::AsyncDieselConnectionManager;
    use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection};
    use mockall::mock;
    use mockall::predicate::*;
    use std::env;
    use std::sync::Arc;
    use uuid::Uuid;

    // Mock the password hasher
    mock! {
        PassWordHasher {
            fn hash_password(&self, password: String) -> String;
        }
    }

    // Mock the database connection pool
    mock! {
        MockPool {
            fn get(&self) -> Result<AsyncPgConnection, diesel::result::Error>;
        }
    }

    // Mock the email service
    mock! {
        BrevoApi {
            fn send_verification_email(&self, email: String, code: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
        }
    }

    #[tokio::test]
    async fn test_signup() {
        // Setup the connection pool
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let config =
            AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
        let pool = Pool::builder(config).build().unwrap();

        // Setup mocks
        let mut mock_hasher = MockPassWordHasher::new();
        mock_hasher
            .expect_hash_password()
            .with(eq("password".to_string()))
            .returning(|_| "hashed_password".to_string());

        let mut mock_brevo_api = MockBrevoApi::new();
        mock_brevo_api
            .expect_send_verification_email()
            .with(always(), always())
            .returning(|_, _| Ok(()));

        // let mut mock_brevo_api = MockBrevoApi::new();
        // mock_brevo_api
        //     .expect_send_verification_email()
        //     .with(eq("test@example.com".to_string()), any())
        //     .returning(|_, _| Ok(()));

        // Setup test data
        let _user_input = UserInput {
            first_name: "Test".to_string(),
            middle_name: None,
            last_name: "User".to_string(),
            username: "testuser".to_string(),
            user_email: "test@example.com".to_string(),
            password_hash: "password".to_string(),
            display_name: None,
            date_of_birth: "2000-01-01".to_string(),
        };

        // Setup the context with mocked dependencies
        let schema = Schema::build(AddUser, EmptyMutation, EmptySubscription)
            .data(Arc::new(mock_hasher))
            .data(Arc::new(pool))
            .data(Arc::new(mock_brevo_api))
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

        // Assert the result
        assert!(result.is_ok());
        // let data = result.data;
        // assert_eq!(data["signup"]["firstName"], "Test");
        // assert_eq!(data["signup"]["lastName"], "User");
    }
}
