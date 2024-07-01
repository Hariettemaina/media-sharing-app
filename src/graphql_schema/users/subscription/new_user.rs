use crate::models::User;
use async_graphql::{Context, Subscription};
use futures_util::lock::Mutex;
use tokio_stream::StreamExt;
use std::sync::Arc;
use tokio::sync::broadcast;

#[derive(Default)]
pub struct GetNewUser;

#[Subscription]
impl GetNewUser {
    pub async fn new_user(&self, ctx: &Context<'_>) -> impl futures_util::Stream<Item = User> {
        let tx = ctx.data::<Arc<Mutex<broadcast::Sender<User>>>>().unwrap();
        let tx = tx.lock().await;

        tokio_stream::wrappers::BroadcastStream::new(tx.subscribe()).map(|result| result.unwrap())
    }
}
