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


// #[derive(Clone, Debug)]
// pub struct Shared<T>(pub Option<SendWrapper<T>>);

// impl<T> Shared<T> {
//     pub fn new(v: T) -> Self {
//         Self(Some(SendWrapper::new(v)))
//     }
// }

// impl<T> Deref for Shared<T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         &*self.0.as_deref().clone().unwrap()
//     }
// }
