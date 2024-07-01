use crate::models::User;
use async_graphql::{Context, InputObject, Object, Result};
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};
use validator::Validate;

#[derive(Default)]
pub struct UpdateUser;

#[derive(Validate, InputObject)]
pub struct UserUpdateInput {
    pub user_id: i32,
    #[validate(length(min = 4, max = 15))]
    pub first_name: Option<String>,
    #[validate(length(min = 4, max = 15))]
    pub last_name: Option<String>,
    #[validate(length(min = 4, max = 15))]
    pub username: Option<String>,
}

#[Object]
impl UpdateUser {
    pub async fn update_user(&self, ctx: &Context<'_>, input: UserUpdateInput) -> Result<User> {
        use crate::schema::users::dsl::{first_name, last_name, username, users};

        let pool: &Pool<AsyncPgConnection> = ctx.data()?;
        let mut conn = pool.get().await?;

        let user = users.find(input.user_id).first::<User>(&mut conn).await?;

        let updated_user = diesel::update(users.find(input.user_id))
            .set((
                first_name.eq(input.first_name.unwrap_or(user.first_name)),
                last_name.eq(input.last_name.unwrap_or(user.last_name)),
                username.eq(input.username.unwrap_or(user.username)),
            ))
            .get_result::<User>(&mut conn)
            .await?;

        Ok(updated_user)
    }
}
