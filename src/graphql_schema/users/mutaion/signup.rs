use async_graphql::{Context, InputObject, Object, Result};

use chrono::NaiveDate;
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;
use validator::{Validate, ValidationError};

use crate::{
    error::PhotoError,
    models::{NewUser, User},
};

#[derive(Default)]
pub struct AddUser;

#[derive(Validate, InputObject)]
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
    pub email_address: String,
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
        let my_uuid = Uuid::new_v4();

        let date_of_birth = NaiveDate::parse_from_str(&input.date_of_birth, "%Y-%m-%d")?;
        let created_at = Naivedate::parse

        let new_user = NewUser {
            first_name: input.first_name,
            middle_name: input.middle_name,
            last_name: input.last_name,
            username: input.username,
            email_address: input.email_address,
            display_name: input.display_name,
            date_of_birth,
            password_hash,
            created_at: todo!(),
            updated_at: todo!(),
        };

        let pool: &Pool<AsyncPgConnection> = ctx.data()?;
        let conn = pool.get().await?;

        let created_user: User = diesel::insert_into(users)
            .values(new_user)
            .get_result(&mut conn)
            .await
            .map_err(|e| {
                log::error!("Failed to register user: {}", e);
                PhotoError::UserAccountAlreadyExists
            })?;

        Ok(created_user)
    }
}
