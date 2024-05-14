#[allow(dead_code)]
use amqprs::{
    callbacks::DefaultConnectionCallback,
    channel::{BasicAckArguments, BasicConsumeArguments, QueueDeclareArguments},
    connection::{Connection, OpenConnectionArguments},
};
use image::GenericImageView;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct MediaProcessingTask {
    media_id: uuid::Uuid,
    file_path: String,
}

async fn process_media(task: &MediaProcessingTask) -> Result<(), Box<dyn Error>> {
    let metadata = image::open(&task.file_path)?;
    let dimensions = metadata.dimensions();
    let resized_image = image::imageops::resize(
        &metadata,
        dimensions.0 / 2,
        dimensions.1 / 2,
        image::imageops::FilterType::Lanczos3,
    );
    let mut resized_image_path = task.file_path.clone();
    resized_image_path.push_str("_resized");
    resized_image.save(&resized_image_path)?;

    // For example:
    // 1. Fetch media metadata from the database
    // 2. Optimize the media file for web viewing
    // 3. Generate different versions for various viewports
    // 4. Update the database with the processed media information

    println!("Processing media: {:?}", task);
    Ok(())
}

#[tokio::main]

async fn main_() -> Result<(), Box<dyn Error>> {
    let connection = Connection::open(&OpenConnectionArguments::new(
        "localhost",
        5672,
        "guest",
        "guest",
    ))
    .await
    .unwrap();
    connection
        .register_callback(DefaultConnectionCallback)
        .await
        .unwrap();
    let channel = connection.open_channel(None).await?;
    
    let queue_name = "media_queue";
    let q_args = QueueDeclareArguments::new(queue_name)
        .durable(true)
        .finish();
    let (_, _, _) = channel.queue_declare(q_args).await.unwrap().unwrap();

    let consumer_args = BasicConsumeArguments::default()
        .queue(String::from(queue_name))
        .finish();
    let (_ctag, mut rx) = channel.basic_consume_rx(consumer_args).await.unwrap();

    while let Some(delivery) = rx.recv().await {
        let task: MediaProcessingTask = serde_json::from_slice(delivery.content.as_ref().unwrap())?;
        process_media(&task).await?;
        channel
            .basic_ack(BasicAckArguments::new(
                delivery.deliver.unwrap().delivery_tag(),
                false,
            ))
            .await
            .unwrap();
    }

    Ok(())
}

// use async_graphql::{Context, InputObject, Object, Result, Upload};
// use chrono::{NaiveDateTime, Utc};
// use diesel::ExpressionMethods;
// use diesel_async::AsyncPgConnection;
// use diesel_async::{pooled_connection::deadpool::Pool, RunQueryDsl};
// use image::{imageops::FilterType, GenericImageView};
// use std::fs::{self, File};
// use std::io::{Read, Write};
// use std::path::Path;
// use uuid::Uuid;

// use crate::PhotoError;

// #[derive(Default)]
// pub struct UploadMedia;

// #[derive(InputObject)]
// pub struct UploadUserInput {
//     pub image: Upload,
//     pub user_id: i32,
// }

// #[Object]
// impl UploadMedia {
//     pub async fn upload(&self, ctx: &Context<'_>, input: UploadUserInput) -> Result<String> {
//         use crate::schema::images;

//         let time_now = Utc::now().naive_utc();

//         let mut image = Vec::new();
//         let mut upload_value = input.image.value(ctx).unwrap();

//         let mut content = upload_value.content; //file data
//         if let Err(e) = content.read_to_end(&mut image) {
//             log::error!("Failed to read image content: {}", e);
//             return Err(async_graphql::Error::new(format!(
//                 "Failed to read image content: {}",
//                 e
//             )));
//         }

//         // Access the filename from the UploadValue
//         let filename = format!(
//             "{}.{}",
//             Uuid::new_v4(),
//             Path::new(&mut upload_value.filename)
//               .extension()
//               .and_then(std::ffi::OsStr::to_str)
//               .unwrap_or("bin") //default value if path does not have an extension
//         );
//         let uploads_dir = "./uploads";
//         let user_uploads_dir = format!("{}/{}", uploads_dir, input.user_id);

//         // Check if the uploads directory exists, if not, create it
//         ifPath::new(uploads_dir).exists() {
//             fs::create_dir_all(uploads_dir).expect("Failed to create uploads directory");
//         }

//         // Check if the user-specific uploads directory exists, if not, create it
//         ifPath::new(&user_uploads_dir).exists() {
//             fs::create_dir_all(&user_uploads_dir)
//               .expect("Failed to create user-specific uploads directory");
//         }

//         // Save the file to the system
//         let filepath = format!("{}/{}", user_uploads_dir, filename);
//         if let Err(e) = File::create(&filepath).and_then(|mut file| file.write_all(&image)) {
//             log::error!("Failed to save file: {}", e);
//             return Err(async_graphql::Error::new(format!(
//                 "Failed to save file: {}",
//                 e
//             )));
//         }

//         // Open the image to get its dimensions and format
//         let img = match image::open(&filepath) {
//             Ok(img) => img,
//             Err(e) => {
//                 log::error!("Failed to open image: {}", e);
//                 return Err(async_graphql::Error::new(format!(
//                     "Failed to open image: {}",
//                     e
//                 )));
//             }
//         };

//         let (width, height) = img.dimensions();
//         let new_width = width / 2;
//         let new_height = height / 2;
//         let resized_img = img.resize(new_width, new_height, FilterType::Triangle);

//         resized_img
//           .save(&filepath)
//           .expect("Failed to save resized imaged");

//         let image_format = image::guess_format(&image).unwrap(); //not supported

//         // Convert the format to a type string
//         let media = match image_format {
//             image::ImageFormat::Png => "image/png",
//             image::ImageFormat::Jpeg => "image/jpeg",
//             image::ImageFormat::Gif => "image/gif",
//             image::ImageFormat::WebP => "image/webp",
//             image::ImageFormat::Pnm => "image/pnm",
//             image::ImageFormat::Tiff => "image/tiff",
//             image::ImageFormat::Tga => "image/tga",
//             image::ImageFormat::Dds => "image/dds",
//             image::ImageFormat::Bmp => "image/bmp",
//             image::ImageFormat::Ico => "image/ico",
//             image::ImageFormat::Hdr => "image/hdr",
//             image::ImageFormat::OpenExr => "image/openexr",
//             image::ImageFormat::Farbfeld => "image/farbfeld",
//             image::ImageFormat::Avif => "image/avif",
//             image::ImageFormat::Qoi => "image/qoi",

//             _ => "None",
//         };

//         let pool: &Pool<AsyncPgConnection> = ctx.data()?;
//         let mut conn = pool.get().await?;

//         // Insert the file's metadata into the database
//         diesel::insert_into(images::table)
//           .values((
//                 images::name.eq(upload_value.filename),
//                 images::file_path.eq(&filepath),
//                 images::description.eq(None::<String>),
//                 images::exif_data.eq(None::<String>),
//                 images::format.eq(media),
//                 images::size.eq(image.len() as i32),
//                 images::width.eq(resized_img.width() as i32),
//                 images::height.eq(resized_img.height() as i32),
//                 images::created_at.eq(time_now),
//                 images::deleted_at.eq(None::<NaiveDateTime>),
//             ))
//           .execute(&mut conn)
//           .await
//           .map_err(|e| {
//                 log::error!("Failed to insert image into database:{}", e);
//                 PhotoError::DatabaseError
//             })?;

//         // RabbitMQ Integration
//         let message = format!("Image with ID {} was uploaded.", Uuid::new_v4());
//         let message_bytes = message.into_bytes();
//         let exchange_name = "exchange_name"; // Replace with your exchange name
//         let routing_key = "routing_key"; // Replace with your routing key

//         // Assuming you have a function to establish RabbitMQ connection and channel
//         // This function should be implemented according to your application's architecture
//         publish_to_rabbitmq(message_bytes, exchange_name, routing_key).await?;

//         Ok(filepath)
//     }
// }

// async fn publish_to_rabbitmq(message_bytes: &[u8], exchange_name: &str, routing_key: &str) -> Result<(), Box<dyn std::error::Error>> {
//     // Your RabbitMQ connection and channel setup here
//     // This is a placeholder for the actual implementation
//     Ok(())
// }
