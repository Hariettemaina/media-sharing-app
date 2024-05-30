use futures_util::Stream;
use tokio::sync::broadcast;
use async_stream::stream;
use std::sync::{Arc, RwLock};

pub type StreamResult<T> = Result<T, async_graphql::Error>;

pub struct PubSub<T> {
    subscribers: Arc<RwLock<Vec<broadcast::Sender<T>>>>,
}

impl<T> PubSub<T>
where
    T: Clone + Send + 'static,
{
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(RwLock::new(vec![])),
        }
    }

    pub fn subscribe(&self, _channel: String) -> impl Stream<Item = StreamResult<T>> {
        let (tx, mut rx) = broadcast::channel(100);
        {
            let mut subscribers = self.subscribers.write().unwrap();
            subscribers.push(tx);
        }

        stream! {
            loop {
                match rx.recv().await {
                    Ok(item) => yield Ok(item),
                    Err(err) => yield Err(async_graphql::Error::new(err.to_string())),
                }
            }
        }
    }

    pub fn publish(&self, item: T) {
        let subscribers = self.subscribers.read().unwrap();
        for tx in subscribers.iter() {
            let _ = tx.send(item.clone());
        }
    }
}

pub async fn get_pubsub_from_ctx<'a, T>(ctx: &'a async_graphql::Context<'a>) -> Result<Arc<RwLock<PubSub<T>>>, async_graphql::Error> 
where
    T: Clone + Send + 'static,
{
    ctx.data::<Arc<RwLock<PubSub<T>>>>()
        .map(Clone::clone)
        .map_err(|err| async_graphql::Error::new(format!("Error: {:?}", err)))
}

// Explanation
// Struct Definition:

// PubSub<T> is a generic struct that maintains a list of subscribers. Each subscriber is a sender (tokio::sync::broadcast::Sender<T>) that can send messages of type T.
// Constructor:
// new() initializes the PubSub with an empty list of subscribers.
// Subscribe Method:
// subscribe method creates a new broadcast channel and adds the sender part (tx) to the list of subscribers.
// It returns a stream that listens for messages on this channel. The stream! macro is used to create an asynchronous stream.
// Publish Method:
// publish method sends a message to all subscribers. Each subscriber's sender (tx) tries to send the message.
// get_pubsub_from_ctx:

// Utility function to extract the PubSub instance from the GraphQL context (ctx).
