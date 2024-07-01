use crate::mailer::BrevoApi;
use crate::models::NewEmailAddress;
use crate::{
    error::PhotoError,
    models::{NewUser, User},
    schema::email_address,
};
use async_graphql::{Context, InputObject, Object, Result};
use chrono::{Duration, NaiveDate, Utc};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};
use std::env;
use tokio::sync::{broadcast, Mutex};
use std::sync::Arc;
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Default)]
pub struct AddUser;

#[derive(Validate, InputObject, Default)]
pub struct UserInput {
    #[validate(length(min = 4, max = 15))]
    pub first_name: String,
    #[validate(length(min = 4, max = 15))]
    pub middle_name: Option<String>,
    #[validate(length(min = 4, max = 15))]
    pub last_name: String,
    #[validate(length(min = 4, max = 15))]
    pub username: String,
    #[validate(email)]
    pub user_email: String,
    pub password_hash: String,
    #[validate(length(min = 4, max = 15))]
    pub display_name: Option<String>,
    #[validate(custom(function = "date_validator"))]
    pub date_of_birth: String,
}

fn date_validator(date: &str) -> Result<(), ValidationError> {
    if let Err(e) = NaiveDate::parse_from_str(date, "%Y-%m-%d") {
        log::error!("Parsing error: {:#?}", e);
        return Err(ValidationError::new("Invalid date provided"));
    };

    Ok(())
}

#[Object]
impl AddUser {
    pub async fn signup(&self, ctx: &Context<'_>, input: UserInput) -> Result<User> {
        use crate::schema::users::dsl::users;
        input.validate()?;
        let hasher = ctx
            .data::<crate::password::PassWordHasher>()
            .map_err(|e| {
                log::error!("Failed to get app data: {:?}", e);
                e
            })
            .unwrap();
        let password_hash = hasher.hash_password(input.password_hash.clone()).unwrap();
        let email_verification_code = Uuid::new_v4();

        let date_of_birth = NaiveDate::parse_from_str(&input.date_of_birth, "%Y-%m-%d")?;
        let now = Utc::now().naive_utc();

        let pool: &Pool<AsyncPgConnection> = ctx.data()?;
        let mut conn = pool.get().await?;

        let new_email = NewEmailAddress {
            email: input.user_email.clone(),
            verification_code: email_verification_code,
            verification_code_expires_at: Utc::now().naive_local() + Duration::hours(24),
        };

        let email_address_id: i32 = diesel::insert_into(email_address::table)
            .values(&new_email)
            .returning(email_address::id)
            .get_result(&mut conn)
            .await
            .map_err(|e| {
                log::error!("Failed to create email: {}", e);
                e
            })?;

        let brevo_api_key = env::var("BREVO_API_KEY").expect("BREVO_API_KEY must be set.");
        let brevo_email = env::var("BREVO_EMAIL").expect("BREVO_EMAIL must be set.");

        let brevo_api = BrevoApi::new(brevo_api_key, brevo_email);
        brevo_api
            .send_verification_email(input.user_email.clone(), email_verification_code)
            .await?;

        let new_user = NewUser {
            first_name: input.first_name,
            middle_name: input.middle_name,
            last_name: input.last_name,
            username: input.username,
            user_email: email_address_id,
            display_name: input.display_name,
            date_of_birth,
            password_hash,
            created_at: now,
            updated_at: now,
        };

        let created_user: User = diesel::insert_into(users)
            .values(new_user)
            .get_result(&mut conn)
            .await
            .map_err(|e| {
                log::error!("Failed to register user: {}", e);
                PhotoError::UserAccountAlreadyExists
            })?;

        if let Ok(tx) = ctx.data::<Arc<Mutex<broadcast::Sender<User>>>>(){
            let tx = tx.lock().await;
        let _ = tx.send(created_user.clone());
        }else {
            log::error!("failed to get user from channel");
        }
        

        Ok(created_user)
    }
}
