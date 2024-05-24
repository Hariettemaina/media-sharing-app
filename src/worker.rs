use amqprs::channel::BasicConsumeArguments;
use amqprs::connection::{Connection, OpenConnectionArguments};
use amqprs::callbacks;

use tokio::task; // for spawning async tasks

async fn consume_messages_from_rabbitmq() -> Result<(), Box<dyn std::error::Error>> {
    let connection = Connection::open(&OpenConnectionArguments::new(
        "localhost",
        5672,
        "guest",
        "guest",
    )).await.unwrap();
    connection.register_callback(callbacks::DefaultConnectionCallback).await?;

    let channel = connection.open_channel(None).await?;
    channel.register_callback(callbacks::DefaultChannelCallback).await?;

    let queue_name = "image_processing_queue";
    let consume_args = BasicConsumeArguments::new("", &queue_name);
    let (_ctag, mut rx)  = channel.basic_consume_rx(consume_args).await?;

    println!("[*] Waiting for messages. To exit press CTRL+C");

    while let Some(message) = rx.recv().await {
        let delivery = message.basic_properties.content.deliver.unwrap();
        let message = String::from_utf8_lossy(&delivery.body);

        // Process the message asynchronously
        task::spawn(async move {
            // Simulate processing delay
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            println!("Processed: {}", message);

            // Acknowledge the message
            channel.basic_ack(delivery.delivery_tag, ).await.unwrap();
        });
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = consume_messages_from_rabbitmq().await {
        println!("Error: {}", e);
    }
}


// **Real-time Updates with WebSockets**

// - **Description:** Implement real-time updates for users using WebSockets.

// - **Expected Functionality:** Users receive real-time notifications when new media is uploaded or when there are updates to their uploaded media.

// - **Criteria for Completion:** Real-time updates are implemented using WebSockets, and users receive notifications in real-time.

// - **Test Suites:**

// - Test WebSocket connection establishment.

// - Test real-time notifications for media uploads and updates.

