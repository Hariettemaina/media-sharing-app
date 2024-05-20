use amqprs::{
    channel::BasicPublishArguments,
    connection::{Connection, OpenConnectionArguments},
    error::Error,
    BasicProperties,
};


pub async fn connect_to_rabbitmq(
    url: &str,
    username: &str,
    password: &str,
) -> Result<Connection, Error> {
    let conn =
        Connection::open(&OpenConnectionArguments::new(url, 5672, username, password)).await?;
    Ok(conn)
}

// Function to publish a message to RabbitMQ
pub async fn publish_to_rabbitmq(queue_name: &str, message: &str) -> Result<(), Error> {
    print!("connecting...");
    let connection_args = OpenConnectionArguments::new("localhost", 5672, "guest", "guest");
    let connection = Connection::open(&connection_args).await?;
    let channel = connection.open_channel(Some(0)).await?;
    
    let publish_args = BasicPublishArguments::new(queue_name, "");
    let properties = BasicProperties::default();
    channel.basic_publish(properties, message.as_bytes().to_vec(), publish_args).await?;
    
    channel.close().await?;
    connection.close().await?;
    Ok(())
}
