use crate::models::User;
use async_graphql::{Context, InputObject, Object, Result};
use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};

#[derive(Default)]
pub struct GetUser;

#[derive(InputObject)]
pub struct GetUserProfile {
    user_id: i32,
}

#[Object]
impl GetUser {
    pub async fn get_user_by_id(&self, ctx: &Context<'_>, input: GetUserProfile) -> Result<User> {
        let pool = ctx.data::<Pool<AsyncPgConnection>>()?;
        let mut connection = pool.get().await?;


        use crate::schema::users::dsl::{id, users};

        let user_profile = FilterDsl::filter(users, id.eq(input.user_id))
            .first::<User>(&mut connection)
            .await
            .map_err(|e| {
                log::error!("Could not get user: {:#?}", e);
                e
            })?;

        Ok(user_profile)
    }
}
