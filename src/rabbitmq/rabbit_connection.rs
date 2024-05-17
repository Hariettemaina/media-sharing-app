use actix_web::body;
use amqprs::callbacks::DefaultConnectionCallback;
use amqprs::channel::BasicConsumeArguments;
use amqprs::connection::{Connection, OpenConnectionArguments};
use amqprs::Deliver;

// pub async fn connect_to_rabbitmq() -> Result<Connection,Error > {
//     let conn = Connection::open(&OpenConnectionArguments::new(
//         "localhost",
//         5672,
//         "guest",
//         "guest",
//     )).await?;
//     Ok(conn)
// }

use amqprs::consumer::DefaultConsumer;
use amqprs::error::Error;
use amqprs::{
    channel::{BasicPublishArguments, QueueDeclareArguments},
    BasicProperties,
};
use image::imageops::FilterType;
use image::GenericImageView;
use tokio::task;
//use async_std::task;

pub async fn publish_task(queue: &str, message: &str) -> Result<Connection, Error> {
    // Establish connection
    let connection = Connection::open(&OpenConnectionArguments::new(
        "localhost",
        5672,
        "guest",
        "guest",
    ))
    .await?;
    connection
        .register_callback(DefaultConnectionCallback)
        .await
        .unwrap();
    // Correctly open a channel using Channel::new
    let channel = connection.open_channel(None).await?;

    // Declare a queue
    channel
        .queue_declare(QueueDeclareArguments::new(queue))
        .await?;

    // Publish message
    let props = BasicProperties::default();
    let publish_args = BasicPublishArguments::new(queue, "");
    channel
        .basic_publish(props, message.as_bytes().to_vec(), publish_args)
        .await?;

    // Close the channel and connection
    channel.close().await?;
    connection.clone().close().await?;
    Ok(connection)
}

pub async fn consume_tasks(queue: &str) -> Result<Connection, Error> {
    // Establish connection
    let connection = Connection::open(&OpenConnectionArguments::new(
        "localhost",
        5672,
        "guest",
        "guest",
    ))
    .await?;

    // Open a channel
    let channel = connection.open_channel(None).await?;

    // Declare a queue
    channel
        .queue_declare(QueueDeclareArguments::new(queue))
        .await?;

    // Set up consumer
    let consumer = DefaultConsumer::new((move |delivery: Deliver| {
        // Assuming `body` is the correct field for the message content
        let message_content = delivery.consumer_tag().as_bytes(); 
        let message = String::from_utf8(message_content.to_vec()).expect("Invalid UTF-8 sequence");
        task::spawn(async move {
            process_message(message).await;
        });
        true // Return true to indicate successful processing
    })(body));

    // Start consuming
    let consume_args = BasicConsumeArguments::new(queue, "consumer");
    channel.basic_consume(consumer, consume_args).await?;

    Ok(connection)
}

pub async fn process_message(message: String) {
    let parts: Vec<&str> = message.split('|').collect();
    let _user_id = parts[0];
    let filepath = parts[1];

    // Perform image processing
    let img = match image::open(filepath) {
        Ok(img) => img,
        Err(e) => {
            log::error!("Failed to open image: {}", e);
            return;
        }
    };

    let (width, height) = img.dimensions();
    let target_widths = [480, 768, 1024, 1024, 1440];

    for &target_width in &target_widths {
        let target_height =
            ((height as f64 / width as f64 * target_width as f64).round() as u32).max(1);
        let resized_img = img.resize_exact(target_width, target_height, FilterType::Triangle);

        let resized_filepath = format!("{}/resized_{}_{}.png", "./uploads", target_width, filepath);
        if let Err(e) = resized_img.save(&resized_filepath) {
            log::error!("Failed to save resized image: {}", e);
        }
    }
}

// Run the consumer in an async-std runtime
#[tokio::main]
async fn main() {
    consume_tasks("image_processing")
        .await
        .expect("Failed to consume tasks");
}
