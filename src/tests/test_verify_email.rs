#[cfg(test)]
mod tests {
    use crate::graphql_schema::users::mutation::verify_email::Verify;
    use async_graphql::{EmptySubscription, Schema};
    use diesel_async::pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager};
    use mockall::{mock, predicate::always};
    use std::env;
    use uuid::Uuid;

    // Mock the BrevoApi
    mock! {
        BrevoApi {
            fn send_verification_email(&self, email: String, code: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
        }
    }

    #[tokio::test]
    async fn test_verify_email() {
        dotenvy::dotenv().ok();
        // Setup the mock BrevoApi
        let mut mock_brevo = MockBrevoApi::new();
        mock_brevo
            .expect_send_verification_email()
            .with(always(), always())
            .returning(|_, _| Ok(()));

        // Setup the database connection pool
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let config =
            AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
        let pool = Pool::builder(config).build().unwrap();
        // Setup the GraphQL schema
        let schema = Schema::build(Verify, Verify, EmptySubscription)
            .data(mock_brevo)
            .data(pool)
            .finish();

        // Execute the verify email mutation
        let result = schema
            .execute(
                r#"
            mutation {
                verifyEmail(input: {
                    code: ""
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


