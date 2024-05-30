use async_graphql::{Context, Subscription};
use futures_util::Stream;

use crate::{
    models::User,
    services::pub_sub::{get_pubsub_from_ctx, StreamResult},
};

#[derive(Default)]
pub struct GetNewUser;

#[Subscription]
impl GetNewUser {
    pub async fn new_user<'a>(
        &'a self,
        ctx: &'a Context<'a>,
        channel: String,
    ) -> impl Stream<Item = StreamResult<User>> + 'a {
        let pub_sub = get_pubsub_from_ctx::<User>(ctx).await.unwrap();
        let pub_sub = pub_sub.write().unwrap();
        pub_sub.subscribe(channel)
    }
}

pub async fn function_to_publish(ctx: &Context<'_>, created_user: User) -> Result<(), async_graphql::Error> {
    let pub_sub = get_pubsub_from_ctx::<User>(ctx).await?;
    let pub_sub = pub_sub.read().unwrap();
    pub_sub.publish(created_user);
    Ok(())
}
