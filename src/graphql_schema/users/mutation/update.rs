use crate::models::User;


use async_graphql::{Context, InputObject, Object, Result};
use diesel::{connection, ExpressionMethods, QueryDsl};
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
    pub middle_name: Option<String>,
    #[validate(length(min = 4, max = 15))]
    pub last_name: Option<String>,
    #[validate(length(min = 4, max = 15))]
    pub username: Option<String>,
}


#[Object]
impl UpdateUser {
    pub async fn update_user(&self, ctx: &Context<'_>, input: UserUpdateInput) -> Result<User> {
        
        let pool: &Pool<AsyncPgConnection> = ctx.data()?;
        let mut conn = pool.get().await?;

        use crate::schema::users::dsl::users;

        let user = users
            .find(input.user_id)
            .first::<User>(&mut connection)
            .await?;

        let updated_user = diesel::update(users.find(input.user_id))
            .set((
                
            ))
            .get_result::<User>(&mut connection)
            .await?;

        Ok(updated_user)
    }
}
