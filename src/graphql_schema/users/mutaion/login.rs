use async_graphql::{Context, InputObject, Object, Result};

use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};
use validator::Validate;

use crate::{error::PhotoError, models::User};

#[derive(Validate, InputObject)]
pub struct LoginInput {
    #[validate(email)]
    pub email_address: String,
    pub password: String,
}
#[derive(Default)]
pub struct Login;

#[Object]
impl Login {
    pub async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<String> {
        use crate::schema::users::dsl::{email_address, users};

        let pool: &Pool<AsyncPgConnection> = ctx.data()?;
        let mut conn = pool.get().await?;

        
        let user: Option<User> = users
            .filter(email_address.eq(input.email_address))
            .first(&mut conn)
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
}
