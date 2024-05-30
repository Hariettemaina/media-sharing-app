use async_graphql::{Context, Subscription};
use futures_util::Stream;

use crate::{
    models::Images,
services::pub_sub::{get_pubsub_from_ctx, StreamResult},
};

#[derive(Default)]
pub struct NewImageSubscription;

#[Subscription]
impl NewImageSubscription {
    pub async fn new_image<'a>(
        &'a self,
        ctx: &'a Context<'a>,
        channel: String,
    ) -> impl Stream<Item = StreamResult<Images>> + 'a {
        let mut pub_sub = get_pubsub_from_ctx::<Images>(ctx).await.unwrap();

        pub_sub.subscribe(channel).await
    }
}