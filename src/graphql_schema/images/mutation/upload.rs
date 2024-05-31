use crate::schema::images;

use crate::PhotoError;
use amqprs::channel::{BasicPublishArguments, QueueDeclareArguments};
use amqprs::connection::{Connection, OpenConnectionArguments};
use amqprs::{callbacks, BasicProperties, DELIVERY_MODE_PERSISTENT};
use async_graphql::futures_util::TryFutureExt;
use async_graphql::{Context, InputObject, Object, Result, Upload};
use chrono::{NaiveDateTime, Utc};
use diesel::ExpressionMethods;
use diesel_async::AsyncPgConnection;
use diesel_async::{pooled_connection::deadpool::Pool, RunQueryDsl};
use image::ImageFormat;
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
        let extension = Path::new(&filename)
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or("bin");
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
            1440, 2650, // Extra large screens
        ];
        let image_format = ImageFormat::from_extension(extension).unwrap();
        let image_format_str = match image_format {
            ImageFormat::Png => "PNG",
            ImageFormat::Jpeg => "JPEG",
            ImageFormat::Gif => "GIF",
            ImageFormat::Bmp => "BMP",
            ImageFormat::Ico => "ICO",
            ImageFormat::Pnm => "PNM",
            ImageFormat::WebP => "WEBP",
            ImageFormat::Hdr => "HDR",
            ImageFormat::Tiff => "TIFF",
            ImageFormat::Tga => "Tga",
            ImageFormat::Dds => "DdS",
            ImageFormat::OpenExr => "OpenEXR",
            ImageFormat::Farbfeld => "Farbfeld",
            ImageFormat::Avif => "AVIF",
            ImageFormat::Qoi => "Qoi",
            _ => "None",
        };
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
                    images::format.eq(image_format_str),
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
                })
                .await?;
        }

        
        
        
        // RabbitMQ publishing logic
        let message = format!(
            "Image uploaded: {}\nPath: {}\nFormat: {}\nSize: {} bytes\nDimensions: {}x{}",
            upload_value.filename,
            filepath,
            image_format_str,
            image_data.len(),
            width,
            height
        );

        send_message_to_rabbitmq(message).await.unwrap();

        Ok(filepath)
    }
}

async fn send_message_to_rabbitmq(message: String) -> Result<(), Box<dyn std::error::Error>> {
    let connection = Connection::open(&OpenConnectionArguments::new(
        "localhost",
        5672,
        "guest",
        "guest",
    ))
    .await
    .unwrap();
    connection
        .register_callback(callbacks::DefaultConnectionCallback)
        .await?;

    // Open a channel
    let channel = connection.open_channel(None).await?;
    channel
        .register_callback(callbacks::DefaultChannelCallback)
        .await?;
    // Declare a queue
    let queue_name = "image_processing_queue";

    // Declare a queue
    let queue_args = QueueDeclareArguments::new(&queue_name);
    channel.queue_declare(queue_args).await?;

    // Publish the message
    let props = BasicProperties::default()
        .with_delivery_mode(DELIVERY_MODE_PERSISTENT)
        .finish();
    let publish_args = BasicPublishArguments::new("", &queue_name);
    channel
        .basic_publish(props, message.into_bytes(), publish_args)
        .await?;
    println!(" [x] Sent \"Hello World!\"");
    channel.close().await?;
    connection.close().await?;
    Ok(())
}
