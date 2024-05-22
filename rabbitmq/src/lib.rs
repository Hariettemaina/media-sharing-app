

pub mod rabbit;

// #[tokio::main]
// async fn main() {
//     let queue_name = "image_processing_queue";
//     let message = "Hello, RabbitMQ";

//     // Establish a single connection to RabbitMQ
//     let connection = match rabbit::connect_to_rabbitmq().await {
//         Ok(connection) => connection,
//         Err(e) => {
//             println!("Failed to connect to RabbitMQ: {}", e);
//             return;
//         },
//     };

//     // Reuse the connection for publishing and consuming
//     match rabbit::publish_to_rabbitmq(&connection.clone(), queue_name, message).await {
//         Ok(_) => println!("Message published"),
//         Err(e) => println!("Failed to publish message: {}", e),
//     };

//     match rabbit::consume_messages( &connection.clone(), queue_name).await {
//         Ok(_) => println!("Consumed messages"),
//         Err(e) => println!("Failed to consume messages: {}", e),
//     };

//     // Close the connection after all operations are done
//     if let Err(e) = connection.close().await {
//         println!("Failed to close connection: {}", e);
//     }
// }

