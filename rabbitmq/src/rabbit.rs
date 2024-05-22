use amqprs::{
    callbacks::DefaultConnectionCallback,
    channel::{BasicConsumeArguments, BasicPublishArguments, QueueDeclareArguments},
    connection::{Connection, OpenConnectionArguments},
    error::Error,
    BasicProperties,
};
// use tokio::sync::Notify;

pub async fn connect_to_rabbitmq() -> Result<Connection, Error> {
    let conn = Connection::open(&OpenConnectionArguments::new(
        "localhost",
        5672,
        "guest",
        "guest",
    ))
    .await?;
    Ok(conn)
}

pub async fn publish_to_rabbitmq(
    connection: &Connection,
    queue_name: &str,
    message: &str,
) -> Result<(), Error> {
    println!("Connecting to channel for publishing...");
    let channel = connection.open_channel(None).await?;

    let publish_args = BasicPublishArguments::new(queue_name, "");
    let properties = BasicProperties::default();

    match channel
        .basic_publish(properties, message.as_bytes().to_vec(), publish_args)
        .await
    {
        Ok(_) => println!("Message published successfully"),
        Err(e) => {
            eprintln!("Failed to publish message: {}", e);
            return Err(e);
        }
    }

    Ok(())
}

pub async fn consume_messages(connection: &Connection, queue_name: &str) -> Result<(), Error> {
    connection
        .register_callback(DefaultConnectionCallback)
        .await?;

    let channel = connection.open_channel(None).await?;

    let q_args = QueueDeclareArguments::default()
        .queue(String::from("image_processing-queue"))
        .durable(false)
        .finish();

    match channel.queue_declare(q_args).await {
        Ok(_) => println!("Queue declared successfully"),
        Err(e) => {
            eprintln!("Failed to declare queue: {}", e);
            return Err(e);
        }
    }

    let consume_args = BasicConsumeArguments::new(queue_name, "consumer");
    let (_ctag, mut rx) = match channel.basic_consume_rx(consume_args).await {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Failed to start consuming: {}", e);
            return Err(e);
        }
    };

    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Some(payload) = msg.content {
                println!(
                    " [x] Received {:?}",
                    std::str::from_utf8(&payload).unwrap_or("Invalid UTF-8")
                );
            }
        }
    });

    // let guard = Notify::new();
    // guard.notified().await;

    Ok(())
}














// async fn publish_image_to_rabbitmq(
//     &self,
//     filepath: String,
// ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
//     dotenvy::dotenv().ok();

//     // Establish a connection
//     let conn = Connection::open(&OpenConnectionArguments::new(
//         "localhost",
//         5672,
//         "guest",
//         "guest",
//     ))
//     .await?;

//     conn.register_callback(DefaultConnectionCallback).await?;

//     // Open a channel
//     let ch = conn.open_channel(None).await?;
//     ch.register_callback(DefaultChannelCallback).await?;

//     // Declare the queue if it doesn't exist
//     let q_args = QueueDeclareArguments::default()
//         .queue(String::from("image_queue"))
//         .durable(true)
//         .finish();
//     let queue_name = match ch.queue_declare(q_args).await? {
//         Some(q) => q.0,
//         None => return Err("Failed to declare queue".into()),
//     };
//     // Prepare the message body
//     let message_body = format!("Image processed: {}", filepath);
//     let payload = message_body.into_bytes();

//     // Set properties for the message
//     let props = BasicProperties::default().with_delivery_mode(2).finish();

//     // Publish the message
//     ch.basic_publish(props, payload, BasicPublishArguments::new("", &queue_name))
//         .await?;

//     println!("Sent image processing status: {}", filepath);

//     // Close the connection
//     conn.close().await?;

//     Ok(filepath)
// }

// async fn start_receiving_images(&self) -> Result<String,Box<dyn std::error::Error + Send + Sync>> {
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
//         Ok(None) => return Err(Box::new(Error::NetworkError("Failed to declare queue".to_string()))),
//         Err(e) => return Err(Box::new(e)),
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

//     Ok( )
// }