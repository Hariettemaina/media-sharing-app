pub mod producer;
pub mod cosumer;


// use crate::cosumer::consume_messages;
// use crate::producer::{connect_to_rabbitmq, publish_task};


// #[tokio::main]
// async fn main() {
//     println!("Connecting.....");
//     connect_to_rabbitmq("localhost", "guest", "guest").await.unwrap();
//     publish_task("image_processing_queue", "hello").await.unwrap();
//     consume_messages("image_processing_queue").await.unwrap();
// }
