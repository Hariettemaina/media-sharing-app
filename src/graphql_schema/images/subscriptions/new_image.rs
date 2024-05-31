use async_graphql::{Context, Subscription};
use async_graphql::futures_util::stream::{self, Stream};
use futures_util::lock::Mutex;
use std::sync::Arc;
use tokio::sync::broadcast;


#[derive(Default)]
pub struct GetNewImage;

#[Subscription]
impl GetNewImage {
    async fn new_image<'a>(&self, ctx: &'a Context<'_>) -> impl Stream<Item = String> + 'a {
        let rx = {
            let tx = ctx.data::<Arc<Mutex<broadcast::Sender<String>>>>().unwrap();
            let tx = tx.lock().await;
            tx.subscribe()
        };

        stream::unfold(rx, |mut rx| async {
            match rx.recv().await {
                Ok(filepath) => Some((filepath, rx)),
                Err(_) => None,
            }
        })
    }
}
