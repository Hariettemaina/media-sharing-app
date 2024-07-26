use amqprs::callbacks::DefaultChannelCallback;
use amqprs::channel::BasicConsumeArguments;
use amqprs::connection::Connection;
use amqprs::connection::OpenConnectionArguments;
use image::GenericImageView;
use std::str;

#[tokio::main]
async fn main() {
    if let Err(e) = consume_messages_from_rabbitmq().await {
        println!("Error: {}", e);
    }
}

async fn consume_messages_from_rabbitmq() -> Result<(), Box<dyn std::error::Error>> {
    let connection_args = OpenConnectionArguments::new("localhost", 5672, "guest", "guest");

    let connection = Connection::open(&connection_args).await?;
    let channel = connection.open_channel(None).await?;
    channel.register_callback(DefaultChannelCallback).await?;

    let consume_args = BasicConsumeArguments::new("image_processing_queue", "");
    let (_ctag, mut rx) = channel.basic_consume_rx(consume_args).await?;

    println!("[*] Waiting for messages. To exit press CTRL+C");

    while let Some(msg) = rx.recv().await {
        if let Some(payload) = msg.content {
            let message_str = str::from_utf8(&payload).unwrap();
            println!(" [x] Received {:?}", message_str);

            // Extract the file path from the message
            if let Some(filepath) = extract_filepath(message_str) {
                process_image(filepath);
            } else {
                println!("Failed to extract filepath from message: {:?}", message_str);
            }
        }
    }

    Ok(())
}

fn extract_filepath(message: &str) -> Option<&str> {
    let path_prefix = "Path: ";
    if let Some(start) = message.find(path_prefix) {
        let path_start = start + path_prefix.len();
        if let Some(end) = message[path_start..].find('\n') {
            return Some(&message[path_start..path_start + end]);
        }
    }
    None
}

fn process_image(filepath: &str) {
    match image::open(filepath) {
        Ok(img) => {
            let thumbnail = img.thumbnail(200, 200);
            let thumbnail_path = format!("{}_thumbnail.png", filepath.trim_end_matches(".png"));
            if let Err(e) = thumbnail.save(&thumbnail_path) {
                println!("Failed to save thumbnail: {}", e);
            } else {
                println!("Thumbnail saved to: {}", thumbnail_path);
            }
        }
        Err(e) => println!("Failed to open image: {}", e),
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
