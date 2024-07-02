use crate::error::PhotoError;
use crate::models::User;
use actix_session::Session;
use async_graphql::{Context, InputObject, Object, Result};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};
use validator::Validate;
use send_wrapper::SendWrapper;
use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct Shared<T>(pub Option<SendWrapper<T>>);

impl<T> Shared<T> {
    pub fn new(v: T) -> Self {
        Self(Some(SendWrapper::new(v)))
    }
}

impl<T> Deref for Shared<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.0.as_deref().clone().unwrap()
    }
}


pub struct RequestContext {
    pub session: Shared<Session>,
}

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
    pub async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<User> {
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
                        let password_hash = user.password_hash.clone();
                        let hasher = ctx
                            .data::<crate::password::PassWordHasher>()
                            .map_err(|e| {
                                log::error!("Failed to get app data: {:?}", e);
                                e
                            })
                            .unwrap();

                        if hasher.verify_password(input.password, password_hash) {
                            // Set the user_id in the session
                            let session = ctx.data::<Shared<Session>>().unwrap();
                            session.insert("user_id", user.id)?;

                            Ok(user)
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
