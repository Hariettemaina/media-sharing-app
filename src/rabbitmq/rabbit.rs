use amqprs::channel::{BasicPublishArguments, QueueDeclareArguments};
use amqprs::callbacks::DefaultChannelCallback;
use amqprs::connection::Connection;
use amqprs::BasicProperties;
use amqprs::error::Error;

pub async fn process_image_and_publish(conn: &Connection, image_data: Vec<u8>) -> Result<(), Error> {
    let ch = conn.open_channel(None).await?;
    ch.register_callback(DefaultChannelCallback).await?;

    let q_name = "image_processing_queue"; // Define your queue name
    let q_args = QueueDeclareArguments::new(q_name).durable(true).finish();
    if let Some((_, _, _)) = ch.queue_declare(q_args.clone()).await? {

    } else {
        ch.queue_declare(q_args).await?;
    }
    let publish_args = BasicPublishArguments::new("", &q_name);
    let props = BasicProperties::default().with_delivery_mode(2).finish(); 
    ch.basic_publish(props, image_data, publish_args).await?;

    Ok(())
}
