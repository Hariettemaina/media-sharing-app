use amqprs::channel::BasicConsumeArguments;
use amqprs::connection::{Connection, OpenConnectionArguments};
use amqprs::callbacks;

//use tokio::task; // for spawning async tasks

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

    while let Some(msg) = rx.recv().await {
        if let Some(payload) = msg.content {
            println!(" [x] Received {:?}", std::str::from_utf8(&payload).unwrap());
            //channel.basic_ack(msg.deliver.delivery_tag, ).await.unwrap();
        }
    };

        // Process the message asynchronously
    //     task::spawn(async move {
    //         // Simulate processing delay
    //         tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    //         println!("Processed: {}", message);

    //         // Acknowledge the message
    //         channel.basic_ack(delivery.delivery_tag, ).await.unwrap();
    //     });
    // }

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

// use actix_web::{web, App, HttpResponse, HttpServer, Responder};
// use actix_web_actors::ws;
// use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
// use async_graphql::{http::GraphiQLSource, Schema};
// use futures::prelude::*;
// use std::sync::Arc;

// // Assuming you have a GraphQL schema defined somewhere
// pub type ApplicationSchema = Schema<async_graphql::EmptyMutation, async_graphql::EmptySubscription, async_graphql::EmptySubscription>;

// async fn index(schema: web::Data<Arc<ApplicationSchema>>, req: HttpRequest, stream: impl Stream<Item = Result<String, std::io::Error>>) -> impl Responder {
//     let mut res = HttpResponse::Ok().into_body_stream(stream);
//     ws::start(res, |socket, msg| {
//         let schema_clone = schema.clone();
//         let fut = async move {
//             match msg {
//                 Ok(ws::Message::Text(text)) => {
//                     let request = GraphQLRequest::new(text.to_string());
//                     let response = schema_clone.execute(request).await.unwrap_or_else(|_| GraphQLResponse::Error("Unknown error".to_string()));
//                     socket.send(response.to_http_response().unwrap_or_else(|_| HttpResponse::InternalServerError().finish())).unwrap();
//                 },
//                 Ok(ws::Message::Ping(msg)) => socket.pong(&msg),
//                 Ok(ws::Message::Close(reason)) => {
//                     socket.close(reason);
//                 },
//                 Err(e) => {
//                     eprintln!("WebSocket error: {}", e);
//                 }
//             }
//         };
//         futures::executor::block_on(fut)
//     }).await
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let schema = Schema::build(async_graphql::EmptyMutation, async_graphql::EmptySubscription, async_graphql::EmptySubscription)
//        .finish();

//     HttpServer::new(move || {
//         App::new()
//            .app_data(web::Data::new(Arc::new(schema)))
//            .route("/", web::get().to(index))
//     })
//    .bind("127.0.0.1:8080")?
//    .run()
//    .await
// }
