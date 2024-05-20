use amqprs::{
    callbacks::DefaultConnectionCallback,
    channel::{ BasicPublishArguments, QueueDeclareArguments},
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


pub async fn publish_task(queue_name: &str, message: &str) -> Result<(), Error> {
    print!("connecting...");
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

    let channel = connection.open_channel(None).await?;


    let q_args = QueueDeclareArguments::default()
        .queue(queue_name.to_owned()) 
        .durable(true)
        .finish();

    if let Some((_, _, _)) = channel.queue_declare(q_args).await.unwrap() {
    } else {
        panic!("Failed to declare queue");
    }

    // Now, publish the message
    let props = BasicProperties::default();
    let publish_args = BasicPublishArguments::new(queue_name, ""); 
    channel
        .basic_publish(props, message.as_bytes().to_vec(), publish_args)
        .await?;

    Ok(())
}
