use async_graphql::{
    futures_util::stream::{Stream, StreamExt},
    Context, SimpleObject, Subscription,
};
use std::sync::Arc;
use tokio::sync::{
    broadcast::{self},
    Mutex,
};
#[derive(Clone, SimpleObject, Debug)]
pub struct MediaUpdate {
    pub user_id: i32,
    pub message: String,
}
#[derive(Default)]
pub struct GetNewImage;

#[Subscription]
impl GetNewImage {
    pub async fn media_updates(&self, ctx: &Context<'_>) -> impl Stream<Item = MediaUpdate> {
        let tx = ctx
            .data::<Arc<Mutex<broadcast::Sender<MediaUpdate>>>>()
            .unwrap();
        let tx = tx.lock().await;

        tokio_stream::wrappers::BroadcastStream::new(tx.subscribe()).map(|result| result.unwrap())
    }
}




// let tx = ctx
// .data::<Arc<Mutex<broadcast::Sender<MediaUpdate>>>>()
// .unwrap();
// let update = MediaUpdate {
// user_id: input.user_id,
// message: "New image uploaded".to_string(),
// };
// tx.lock().await.send(update).unwrap();
