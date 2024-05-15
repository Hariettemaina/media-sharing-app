use amqprs::connection::{Connection, OpenConnectionArguments};
use amqprs::error::Error;

pub async fn connect_to_rabbitmq() -> Result<Connection,Error > {
    let conn = Connection::open(&OpenConnectionArguments::new(
        "localhost",
        5672,
        "guest", 
        "guest", 
    )).await?;
    Ok(conn)
}
