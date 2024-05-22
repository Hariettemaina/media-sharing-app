use amqprs::{
    callbacks::DefaultConnectionCallback,
    channel::{BasicConsumeArguments, BasicPublishArguments, QueueDeclareArguments},
    connection::{Connection, OpenConnectionArguments},
    error::Error,
    BasicProperties,
};
use tokio::sync::Notify;
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

    let guard = Notify::new();
    guard.notified().await;

    Ok(())
}


#[tokio::main]
async fn main() {
    let queue_name = "image_processing_queue";
    let message = "Hello, RabbitMQ";

    // Establish a single connection to RabbitMQ
    let connection = match connect_to_rabbitmq().await {
        Ok(connection) => connection,
        Err(e) => {
            println!("Failed to connect to RabbitMQ: {}", e);
            return;
        },
    };

    // Reuse the connection for publishing and consuming
    match publish_to_rabbitmq(&connection.clone(), queue_name, message).await {
        Ok(_) => println!("Message published"),
        Err(e) => println!("Failed to publish message: {}", e),
    };

    match consume_messages( &connection.clone(), queue_name).await {
        Ok(_) => println!("Consumed messages"),
        Err(e) => println!("Failed to consume messages: {}", e),
    };

    // Close the connection after all operations are done
    if let Err(e) = connection.close().await {
        println!("Failed to close connection: {}", e);
    }
}