// use amqprs::callbacks::DefaultChannelCallback;
// use amqprs::channel::{BasicPublishArguments, QueueDeclareArguments};
// use amqprs::connection::Connection;
// use amqprs::error::Error;
// use amqprs::BasicProperties;

// pub async fn process_image_and_publish(
//     conn: &Connection,
//     image_data: Vec<u8>,
// ) -> Result<(), Error> {
//     let ch = conn.open_channel(None).await?;
//     ch.register_callback(DefaultChannelCallback).await?;

//     let q_name = "image_processing_queue"; // Define your queue name
//     let q_args = QueueDeclareArguments::new(q_name).durable(true).finish();
//     if let Some((_, _, _)) = ch.queue_declare(q_args.clone()).await? {
//     } else {
//         ch.queue_declare(q_args).await?;
//     }
//     let publish_args = BasicPublishArguments::new("", &q_name);
//     let props = BasicProperties::default().with_delivery_mode(2).finish();
//     ch.basic_publish(props, image_data, publish_args).await?;

//     Ok(())
// }

// use amqprs::channel::BasicConsumeArguments;

// async fn consume_images(conn: &Connection) -> Result<(), Error> {
//     let channel = conn.open_channel(None).await?;
//     let queue_name = "image_processing_queue";

//     let consumer_args = BasicConsumeArguments::default()
//         .queue(queue_name.to_string())
//         .manual_ack(false)
//         .finish();

//     let (_consumer_tag, mut rx) = channel.basic_consume_rx(consumer_args).await?;
//     while let Some(result) = rx.recv().await {
//         match result {
//             Ok(message) => {
//                 let image_data = message.content.unwrap();
//                 process_image_data(&image_data).await.unwrap(); // Assuming this function is implemented elsewhere
//                 channel.basic_ack(message.delivery_tag()).await.unwrap();
//             }
//             Err(error) => {
//                 // Handle the error case
//                 eprintln!("Error consuming message: {}", error);
//             }
//         }
//     }

//     Ok(())
// }
// //

