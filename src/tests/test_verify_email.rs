#[cfg(test)]
mod tests {
    use crate::graphql_schema::users::mutation::verify_email::{Verify, VerifyEmail};
    use async_graphql::{EmptyMutation, EmptySubscription, Schema};
    use diesel_async::pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager};
    use mockall::{mock, predicate::always};
    use std::{env, sync::Arc};
    use uuid::Uuid;

    // Mock the BrevoApi
    mock! {
        BrevoApi {
            fn send_verification_email(&self, email: String, code: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
        }
    }

    #[tokio::test]
    async fn test_verify_email() {
        // Setup the mock BrevoApi
        let mut mock_brevo = MockBrevoApi::new();
        mock_brevo
            .expect_send_verification_email()
            .with(always(), always())
            .returning(|_, _| Ok(()));

        // Setup the mock database connection pool
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let config =
            AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
        let pool = Pool::builder(config).build().unwrap();
        // Setup the GraphQL schema
        let schema = Schema::build(Verify, EmptyMutation, EmptySubscription)
            .data(Arc::new(mock_brevo))
            .data(Arc::new(pool))
            .finish();

        // Create the verify email input
        let _verify_email_input = VerifyEmail {
            code: Uuid::new_v4().to_string(),
        };

        // Execute the verify email mutation
        let result = schema
            .execute(
                r#"
            mutation {
                verifyEmail(input: {
                    code: "your_uuid_here"
                }) {
                    verified
                }
            }
        "#,
            )
            .await;

        // Assert the result
        assert!(result.is_ok());
        // let data = result.data;
        // assert_eq!(data["verifyEmail"]["verified"], true);
    }
}
