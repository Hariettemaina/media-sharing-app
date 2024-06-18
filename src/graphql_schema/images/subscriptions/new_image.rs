use async_graphql::{
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
impl GetNewImage  {
    async fn media_updates(&self, ctx: &Context<'_>) -> impl futures_util::Stream<Item = MediaUpdate> {
        let tx = ctx.data_unchecked::<Arc<Mutex<broadcast::Sender<MediaUpdate>>>>().clone();
        let rx = tx.lock().await.subscribe();
        futures_util::stream::unfold(rx, |mut rx| async {
            match rx.recv().await {
                Ok(msg) => Some((msg, rx)),
                Err(_) => None,
            }
        })
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


// impl GetNewImage {
//     pub async fn media_updates(&self, ctx: &Context<'_>) -> impl Stream<Item = MediaUpdate> {
//         let tx = ctx
//             .data::<Arc<Mutex<broadcast::Sender<MediaUpdate>>>>()
//             .unwrap();
//         let tx = tx.lock().await;

//         tokio_stream::wrappers::BroadcastStream::new(tx.subscribe()).map(|result| result.unwrap())
//     }
// }

