use crate::models::User;
use async_graphql::futures_util::stream::{self, Stream};
use async_graphql::{Context, Subscription};
use futures_util::lock::Mutex;
use std::sync::Arc;
use tokio::sync::broadcast;

#[derive(Default)]
pub struct GetNewUser;

#[Subscription]
impl GetNewUser {
    async fn new_user<'a>(&self, ctx: &'a Context<'_>) -> impl Stream<Item = User> + 'a {
        let rx = {
            let tx = ctx.data::<Arc<Mutex<broadcast::Sender<User>>>>().unwrap();
            let tx = tx.lock().await;
            tx.subscribe()
        };

        stream::unfold(rx, |mut rx| async {
            match rx.recv().await {
                Ok(user) => Some((user, rx)),
                Err(_) => None,
            }
        })
    }
}
