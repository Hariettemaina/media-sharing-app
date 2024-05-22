// // // use amqprs::{
// // //     callbacks,
// // //     channel::{BasicPublishArguments, QueueDeclareArguments},
// // //     connection::{Connection, OpenConnectionArguments},
// // //     error::Error,
// // //     BasicProperties,
// // // };
// // // use std::sync::Arc;
// // // use tokio::sync::Mutex;

// // // pub struct ImageUploadProducer {
// // //     connection: Arc<Mutex<Connection>>,
// // // }

// // // impl ImageUploadProducer {
// // //     pub async fn new() -> Result<Self, Error> {
        
// // //         let args = OpenConnectionArguments::new("localhost", 5672, "guest", "guest");
// // //         match Connection::open(&args).await {
// // //             Ok(connection) => {
// // //                 if let Err(err) = connection
// // //                     .register_callback(callbacks::DefaultConnectionCallback)
// // //                     .await
// // //                 {
// // //                     log::error!("Failed to register connection callback: {}", err);
// // //                     return Err(err);
// // //                 }

// // //                 Ok(Self {
// // //                     connection: Arc::new(Mutex::new(connection)),
// // //                 })
// // //             }
// // //             Err(err) => {
// // //                 log::error!("Failed to open connection: {}", err);
// // //                 Err(err)
// // //             }
// // //         }
// // //     }

// // //     pub async fn publish_upload(&self, image_id: String, temp_path: String) -> Result<(), Error> {
// // //         let connection = self.connection.lock().await;
// // //         let channel = match connection.open_channel(None).await {
// // //             Ok(channel) => channel,
// // //             Err(err) => {
// // //                 log::error!("Failed to open channel: {}", err);
// // //                 return Err(err);
// // //             }
// // //         };

// // //         let q_name = "image_processing";
// // //         let payload = format!(
// // //             "{{\"image_id\": \"{}\", \"temp_path\": \"{}\"}}",
// // //             image_id, temp_path
// // //         );
// // //         let q_args = QueueDeclareArguments::new(q_name).durable(true).finish();
// // //         if let Err(err) = channel.queue_declare(q_args).await {
// // //             log::error!("Failed to declare queue: {}", err);
// // //             return Err(err);
// // //         }

// // //         let publish_args = BasicPublishArguments::new("", &q_name);
// // //         let props = BasicProperties::default().with_delivery_mode(2).finish();

// // //         if let Err(err) = channel
// // //             .basic_publish(props, payload.as_bytes().to_vec(), publish_args)
// // //             .await
// // //         {
// // //             log::error!("Failed to publish message: {}", err);
// // //             return Err(err);
// // //         }

// // //         Ok(())
// // //     }
// // // }



// // #![allow(unused)]
// // use amqprs::{
// //     callbacks,
// //     channel::{BasicPublishArguments, QueueDeclareArguments},
// //     connection::{Connection, OpenConnectionArguments},
// //     error::Error,
// //     BasicProperties,
// // };
// // use std::sync::Arc;
// // use tokio::sync::Mutex;

// // pub struct AmqpHandler {
// //     connection: Arc<Mutex<Connection>>,
// // }

// // impl AmqpHandler {
// //     pub async fn new(addr: &str, username: &str, password: &str) -> Result<Self, Error> {
// //         let args = OpenConnectionArguments::new(addr, 5672, username, password);
// //         match Connection::open(&args).await {
// //             Ok(connection) => {
// //                 if let Err(err) = connection.register_callback(callbacks::DefaultConnectionCallback).await {
// //                     log::error!("Failed to register connection callback: {}", err);
// //                     return Err(err);
// //                 }

// //                 Ok(Self {
// //                     connection: Arc::new(Mutex::new(connection)),
// //                 })
// //             },
// //             Err(err) => {
// //                 log::error!("Failed to open connection: {}", err);
// //                 Err(err)
// //             }
// //         }
// //     }

// //     async fn create_publisher(&self, queue: &str) -> Result<(), Error> {
// //         let connection = self.connection.lock().await;
// //         let channel = match connection.open_channel(None).await {
// //             Ok(channel) => channel,
// //             Err(err) => {
// //                 log::error!("Failed to open channel: {}", err);
// //                 return Err(err);
// //             }
// //         };

// //         let q_args = QueueDeclareArguments::new(queue).durable(true).finish();
// //         if let Err(err) = channel.queue_declare(q_args).await {
// //             log::error!("Failed to declare queue: {}", err);
// //             return Err(err);
// //         }

// //         Ok(())
// //     }

// //     pub async fn publish_image(&self, filepath: String) -> Result<(), Error> {
// //         let connection = self.connection.lock().await;
// //         let channel = match connection.open_channel(None).await {
// //             Ok(channel) => channel,
// //             Err(err) => {
// //                 log::error!("Failed to open channel: {}", err);
// //                 return Err(err);
// //             }
// //         };

// //         let q_name = "image_processing";
// //         let payload = filepath;
// //         let q_args = QueueDeclareArguments::new(q_name).durable(true).finish();
// //         if let Err(err) = channel.queue_declare(q_args).await {
// //             log::error!("Failed to declare queue: {}", err);
// //             return Err(err);
// //         }

// //         let publish_args = BasicPublishArguments::new("", &q_name);
// //         let props = BasicProperties::default().with_delivery_mode(2).finish();

// //         if let Err(err) = channel.basic_publish(props, payload.as_bytes().to_vec(), publish_args).await {
// //             log::error!("Failed to publish message: {}", err);
// //             return Err(err);
// //         }

// //         log::info!("Published image URL to queue");

// //         Ok(())
// //     }
// // }



// use amqprs::{
//     callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
//     channel::{BasicConsumeArguments, QueueDeclareArguments},
//     connection::{Connection, OpenConnectionArguments},
//     error::Error,
// };
// use tokio::{self, sync::Notify};


// pub async fn start_receiving_images() -> Result<(), Error> {
//     dotenvy::dotenv().ok();

//     // Establish a connection
//     let conn = Connection::open(&OpenConnectionArguments::new(
//         "localhost",
//         5672,
//         "guest",
//         "guest",
//     ))
//     .await
//     .map_err(|e| e)?;

//     conn.register_callback(DefaultConnectionCallback)
//         .await
//         .map_err(|e| e)?;

//     // Open a channel
//     let ch = conn.open_channel(None).await.map_err(|e| e)?;
//     ch.register_callback(DefaultChannelCallback)
//         .await
//         .map_err(|e| e)?;

//     // Declare the queue if it doesn't exist
//     let q_args = QueueDeclareArguments::default()
//         .queue(String::from("image_queue"))
//         .durable(true)
//         .finish();
//     let queue_name = match ch.queue_declare(q_args).await {
//         Ok(Some(q)) => q.0,
//         Ok(None) => return Err(Error::NetworkError("Failed to declare queue".to_string())),
//         Err(e) => return Err(e),
//     };

//     // Consume messages from the queue
//     let consumer_args = BasicConsumeArguments::new(&queue_name, "receiver.rs");
//     let (_ctag, mut rx) = ch.basic_consume_rx(consumer_args).await.map_err(|e| e)?;

//     // Spawn a separate task to consume messages
//     let guard = Notify::new();
//     tokio::spawn(async move {
//         while let Some(msg) = rx.recv().await {
//             if let Some(payload) = msg.content {
//                 let message = std::str::from_utf8(&payload).unwrap_or_default();
//                 println!("Received message: {}", message);
//                 // Process the received message here
//                 // For example, you could parse the message and update the database or trigger further processing
//             }
//         }
//     });

//     println!(" [*] Waiting for messages. To exit press CTRL+C");

//     // Wait for a signal to stop the receiver
//     guard.notified().await;

//     Ok(())
// }
