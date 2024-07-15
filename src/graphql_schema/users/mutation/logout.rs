use actix_session::Session;
use async_graphql::{Context, Object, Result};
use crate::graphql_schema::users::mutation::login::Shared;


#[derive(Default)]
pub struct Logout;

#[Object]
impl Logout {
    pub async fn logout(&self, ctx: &Context<'_>) -> Result<Option<String>> {
        let session = ctx.data::<Shared<Session>>().unwrap();

        let user_id = session.remove("user_id");
        Ok(user_id.map(|id| format!("User {} logged out successfully", id)))
    }
}


