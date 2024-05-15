// // use amqprs::{
// //     callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
// //     channel::{BasicPublishArguments, QueueDeclareArguments},
// //     connection::{Connection, OpenConnectionArguments},
// //     BasicProperties,
// // };

// use async_graphql::{Context, InputObject, Object, Result, Upload};
// use chrono::{NaiveDateTime, Utc};
// use diesel::ExpressionMethods;
// use diesel_async::{AsyncConnection, AsyncPgConnection};
// use diesel_async::{pooled_connection::deadpool::Pool, RunQueryDsl};
// use image::{imageops::FilterType, GenericImageView};
// // use serde::{Deserialize, Serialize};
// use std::fs::{self, File};
// use std::io::{Read, Write};
// use std::path::Path;
// use uuid::Uuid;

// use crate::PhotoError;

// // #[derive(Serialize, Deserialize)]
// // struct MediaProcessingTask {
// //     media_id: uuid::Uuid,
// //     file_path: String,
// // }

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

//         // access the filename from the UploadValue    (each uploaded file has a unique name, even if two files have the same original name.uuid)
//         let filename = format!(
//             "{}.{}",
//             Uuid::new_v4(),
//             Path::new(&mut upload_value.filename)
//                 .extension()
//                 .and_then(std::ffi::OsStr::to_str)
//                 .unwrap_or("bin") //default value if path does not have an extension
//         );
//         let uploads_dir = "./uploads";
//         let user_uploads_dir = format!("{}/{}", uploads_dir, input.user_id);

//         // Check if the uploads directory exists, if not, create it
//         if !Path::new(uploads_dir).exists() {
//             fs::create_dir_all(uploads_dir).expect("Failed to create uploads directory");
//         }

//         // Check if the user-specific uploads directory exists, if not, create it
//         if !Path::new(&user_uploads_dir).exists() {
//             fs::create_dir_all(&user_uploads_dir)
//                 .expect("Failed to create user-specific uploads directory");
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

//         // Define the target widths for each viewport category
//         let target_widths = [
//             480,  // Mobile devices
//             768,  // iPads/Tablets
//             1024, // Desktops
//             1024, // Laptops
//             1440, // Extra large screens
//         ];

//         // Process each viewport category
//         for target_width in &target_widths {
//             let target_height =
//                 ((height as f64 / width as f64 * *target_width as f64).round() as u32).max(1); // Ensure height is at least 1 pixel
//             let resized_img = img.resize_exact(*target_width, target_height, FilterType::Triangle);

//             // Save the resized image
//             let resized_filepath = format!(
//                 "{}/resized_{}_{}.png",
//                 user_uploads_dir, filename, target_width
//             );
//             if let Err(e) = resized_img.save(&resized_filepath) {
//                 log::error!("Failed to save resized image: {}", e);
//                 return Err(async_graphql::Error::new(format!(
//                     "Failed to save resized image: {}",
//                     e
//                 )));
//             }

//             let pool: &Pool<AsyncPgConnection> = ctx.data()?;
//             let mut conn = pool.get().await?;

//             // Insert the file's metadata into the database
//             // Start a transaction
//             conn.transaction(|| {
//                 diesel::insert_into(images::table)
//                     .values((
//                         images::name.eq(filename.clone()),
//                         images::file_path.eq(&resized_filepath),
//                         images::description.eq(Some("Resized image".into())),
//                         images::exif_data.eq(None::<String>),
//                         images::format.eq("image/png"),
//                         images::size.eq(image_data.len() as i32),
//                         images::width.eq(resized_img.width() as i32),
//                         images::height.eq(resized_img.height() as i32),
//                         images::created_at.eq(time_now),
//                         images::deleted_at.eq(None::<NaiveDateTime>),
//                     ))
//                     .execute(&mut conn)
//                     .map_err(|e| {
//                         log::error!("Failed to insert image into database: {}", e);
//                         PhotoError::DatabaseError
//                     })
//             })
//             .await?;
//         }
// //Publish the media processing task to RabbitMQ
// let conn = Connection::open(&OpenConnectionArguments::new(
//     "localhost",
//     5672,
//     "guest",
//     "guest",
// ))
// .await
// .unwrap();
// conn.register_callback(DefaultConnectionCallback)
//     .await
//     .unwrap();

// let channel = conn.open_channel(None).await?;
// channel
//     .register_callback(DefaultChannelCallback)
//     .await
//     .unwrap();

// let queue_name = "media_queue";
// let q_args = QueueDeclareArguments::new(queue_name)
//     .durable(true)
//     .finish();

// let (_, _, _) = channel.queue_declare(q_args).await.unwrap().unwrap();

// // let queue = channel
// //     .queue_declare(queue_name)
// //     .await?;

// let media_task = MediaProcessingTask {
//     media_id: Uuid::new_v4(),
//     file_path: filepath.clone(),
// };

// let message_bytes = serde_json::to_vec(&media_task)?;
// let _message_str =
//     String::from_utf8(message_bytes.clone()).expect("Failed to convert bytes to string");
// channel
//     .basic_publish(BasicProperties::default(), message_bytes, BasicPublishArguments::new("", &queue_name))
//     .await?;

//         Ok(filepath)
//     }
// }
//upload_value
// pub filename: String,
// pub content_type: Option<String>,
// pub content: File,

// 2. *Media Upload*
//    - *Description:* Allow users to upload images and videos to the platform.
//    - *Expected Functionality:* Users can select and upload media files from their devices. Uploaded media is stored in the file system and metadata is stored in the database.
//    - *Criteria for Completion:* Users can successfully upload images and videos. Media files are stored in the file system and metadata is stored in the database.
//    - *Test Suites:*
//      - Test file upload functionality.
//      - Test database storage of metadata for uploaded media.
// let mut img = img.resize(800, 800, image::imageops::FilterType::Nearest);
// img = img.adjust_contrast(1.2);
// img.save_with_format(filepath, image::ImageFormat::Jpeg, 80).unwrap();
//let mut img = img.resize(800, 800, FilterType::Nearest);
//img = img.adjust_contrast(1.2);

// 3. *Media Optimization*
//    - *Description:* Automatically optimize uploaded images and videos for web viewing and different viewports.
//    - *Expected Functionality:* Upon upload, media files are processed to reduce file size while maintaining quality suitable for web viewing. Different versions of media files optimized for various viewports are generated.
//    - *Criteria for Completion:* Uploaded media files are optimized for web viewing and different viewports.
//    - *Test Suites:*
//      - Test image optimization process.
//      - Test video optimization process.
//      - Test generation of different viewport versions.


use crate::schema::images;
use crate::PhotoError;
use async_graphql::futures_util::TryFutureExt;
use async_graphql::{Context, InputObject, Object, Result, Upload};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel_async::{pooled_connection::deadpool::Pool, RunQueryDsl};
use diesel_async::AsyncPgConnection;
use image::{imageops::FilterType, GenericImageView};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use uuid::Uuid;

#[derive(Default)]
pub struct UploadMedia;

#[derive(InputObject)]
pub struct UploadUserInput {
    pub image: Upload,
    pub user_id: i32,
}

#[Object]
impl UploadMedia {
    pub async fn upload(&self, ctx: &Context<'_>, input: UploadUserInput) -> Result<String> {
        let time_now = Utc::now().naive_utc();

        let mut image_data = Vec::new();
        let mut upload_value = input.image.value(ctx).unwrap();

        let mut content = upload_value.content; //file data
        if let Err(e) = content.read_to_end(&mut image_data) {
            log::error!("Failed to read image content: {}", e);
            return Err(async_graphql::Error::new(format!(
                "Failed to read image content: {}",
                e
            )));
        }

        let filename = format!(
            "{}.{}",
            Uuid::new_v4(),
            Path::new(&mut upload_value.filename)
                .extension()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or("bin") //default value if path does not have an extension
        );
        let uploads_dir = "./uploads";
        let user_uploads_dir = format!("{}/{}", uploads_dir, input.user_id);

        if !Path::new(uploads_dir).exists() {
            fs::create_dir_all(uploads_dir).expect("Failed to create uploads directory");
        }

        if !Path::new(&user_uploads_dir).exists() {
            fs::create_dir_all(&user_uploads_dir)
                .expect("Failed to create user-specific uploads directory");
        }

        let filepath = format!("{}/{}", user_uploads_dir, filename);
        if let Err(e) = File::create(&filepath).and_then(|mut file| file.write_all(&image_data)) {
            log::error!("Failed to save file: {}", e);
            return Err(async_graphql::Error::new(format!(
                "Failed to save file: {}",
                e
            )));
        }

        let img = match image::open(&filepath) {
            Ok(img) => img,
            Err(e) => {
                log::error!("Failed to open image: {}", e);
                return Err(async_graphql::Error::new(format!(
                    "Failed to open image: {}",
                    e
                )));
            }
        };

        let (width, height) = img.dimensions();

        // Define the target widths for each viewport category
        let target_widths = [
            480,  // Mobile devices
            768,  // iPads/Tablets
            1024, // Desktops
            1024, // Laptops
            1440, // Extra large screens
        ];

        // Process each viewport category
        for target_width in &target_widths {
            let target_height =
                ((height as f64 / width as f64 * *target_width as f64).round() as u32).max(1); // Ensure height is at least 1 pixel
            let resized_img = img.resize_exact(*target_width, target_height, FilterType::Triangle);

            // Save the resized image
            let resized_filepath = format!(
                "{}/resized_{}_{}.png",
                user_uploads_dir, filename, target_width
            );
            if let Err(e) = resized_img.save(&resized_filepath) {
                log::error!("Failed to save resized image: {}", e);
                return Err(async_graphql::Error::new(format!(
                    "Failed to save resized image: {}",
                    e
                )));
            }

            
            let pool: &Pool<AsyncPgConnection> = ctx.data()?; 
            let mut conn = pool.get().await?;

            diesel::insert_into(images::table)
                .values((
                    images::name.eq(filename.clone()),
                    images::file_path.eq(&resized_filepath),
                    images::description.eq(Some("Resized image".to_string())),
                    images::exif_data.eq(None::<String>),
                    images::format.eq("image/png"),
                    images::size.eq(image_data.len() as i32),
                    images::width.eq(resized_img.width() as i32),
                    images::height.eq(resized_img.height() as i32),
                    images::created_at.eq(time_now),
                    images::deleted_at.eq(None::<NaiveDateTime>),
                ))
                .execute(&mut conn)
                .map_err(|e| {
                    log::error!("Failed to insert image into database: {}", e);
                    PhotoError::DatabaseError
                }).await?;
        }

        Ok(filepath)
    }
}
