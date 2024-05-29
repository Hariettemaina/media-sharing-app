use std::sync::Arc;
use async_graphql::{SimpleObject, Subscription, Context};
use async_graphql::futures_util::stream::Stream;
use futures::stream;
use tokio::sync::Mutex;
use tokio::sync::broadcast;

#[derive(SimpleObject, Clone)]
pub struct Image {
    pub url: String,
    pub description: String,
}

#[derive(Default)]
pub struct Subscription {
    pub image_sender: Arc<Mutex<Option<broadcast::Sender<Image>>>>,
}

#[Subscription]
impl Subscription {
    pub async fn new_image(&self,ctx: &Context<'_>) -> impl Stream<Item = Image> {
        let (sender, receiver) = broadcast::channel(100);
        *self.image_sender.lock().await = Some(sender);
        stream::unfold(receiver, |mut receiver| async {
            match receiver.recv().await {
                Ok(image) => Some((image, receiver)),
                Err(_) => None,
            }
        })
    }
}
