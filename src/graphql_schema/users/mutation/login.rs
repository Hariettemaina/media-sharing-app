use async_graphql::{Context, InputObject, Object, Result};

use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};
use validator::Validate;

use crate::{error::PhotoError, models::User};

#[derive(Validate, InputObject)]
pub struct LoginInput {
    #[validate(email)]
    pub user_email: String,
    pub password: String,
}
#[derive(Default)]
pub struct Login;

#[Object]
impl Login {
    pub async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<String> {
        use crate::schema::{email_address, users};

        let pool: &Pool<AsyncPgConnection> = ctx.data()?;
        let mut conn = pool.get().await?;

        let email_address_id: Option<i32> = email_address::table
            .filter(email_address::email.eq(input.user_email))
            .select(email_address::id)
            .get_result(&mut conn)
            .await
            .optional()?;

        match email_address_id {
            Some(email_address_id) => {
                let user: Option<User> = users::table
                    .filter(users::user_email.eq(email_address_id))
                    .get_result(&mut conn)
                    .await
                    .optional()?;

                match user {
                    Some(user) => {
                        let hasher = ctx
                            .data::<crate::password::PassWordHasher>()
                            .map_err(|e| {
                                log::error!("Failed to get app data: {:?}", e);
                                e
                            })
                            .unwrap();

                        if hasher.verify_password(input.password, user.password_hash) {
                            Ok("User authenticated".to_string())
                        } else {
                            Err(PhotoError::InvalidCredentials.into())
                        }
                    }
                    None => Err(PhotoError::UserNotFound.into()),
                }
            }
            None => Err(PhotoError::UserNotFound.into()),
        }
    }
}
