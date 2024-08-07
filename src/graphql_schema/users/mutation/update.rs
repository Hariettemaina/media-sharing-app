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

        input.validate()?;

        let pool: &Pool<AsyncPgConnection> = ctx.data()?;
        let mut conn = pool.get().await?;

        let user: User = users.find(input.user_id).first(&mut conn).await?;

        let updated_user = diesel::update(users.find(input.user_id))
            .set((
                first_name.eq(input.first_name.unwrap_or_else(|| user.first_name.clone())),
                last_name.eq(input.last_name.unwrap_or_else(|| user.last_name.clone())),
                username.eq(input.username.unwrap_or_else(|| user.username.clone())),
            ))
            .get_result::<User>(&mut conn)
            .await?;

        Ok(updated_user)
    }
}