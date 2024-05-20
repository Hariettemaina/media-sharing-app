use amqprs::{
    callbacks::DefaultConnectionCallback,
    channel::{ BasicConsumeArguments, QueueDeclareArguments},
    connection::{Connection, OpenConnectionArguments},
    error::Error,
};
use tokio::sync::Notify;


pub async fn consume_messages(queue_name: &str) -> Result<(), Error> {
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
        .queue(String::from(queue_name))
        .durable(false)
        .finish();

    let (queue_name, _, _) = channel.queue_declare(q_args).await.unwrap().unwrap();

    let consume_args = BasicConsumeArguments::new(&queue_name, "consumer");
//  UnboundedReceiver
    let (_ctag, mut rx) = channel.basic_consume_rx(consume_args).await.unwrap();
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Some(payload) = msg.content {
                println!(" [x] Received {:?}", std::str::from_utf8(&payload).unwrap());
            }
        }
    });

    let guard = Notify::new();
    guard.notified().await;

    Ok(())
}