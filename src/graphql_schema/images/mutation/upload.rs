use async_graphql::{Context, InputObject, Object, Result, Upload};
use chrono::{NaiveDateTime, Utc};
use diesel::ExpressionMethods;
use diesel_async::AsyncPgConnection;
use diesel_async::{pooled_connection::deadpool::Pool, RunQueryDsl};
use image::GenericImageView;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use uuid::Uuid;

use crate::schema::images;
use crate::PhotoError;

#[derive(Default)]
pub struct UploadMedia;

#[derive(InputObject)]
pub struct UserInput {
    pub image: Upload,
    pub user_id: i32,
}

#[Object]
impl UploadMedia {
    pub async fn upload(&self, ctx: &Context<'_>, input: UserInput) -> Result<bool> {
        let time_now = Utc::now().naive_utc();

        let mut image = Vec::new();
        let mut upload_value = input.image.value(ctx).unwrap();
        let mut content = upload_value.content;
        if let Err(e) = content.read_to_end(&mut image) {
            log::error!("Failed to read image content: {}", e);
            return Err(async_graphql::Error::new(format!("Failed to read image content: {}", e)));
        }

        // access the filename from the UploadValue    (each uploaded file has a unique name, even if two files have the same original name.uuid)
        let filename = format!(
            "{}.{}",
            Uuid::new_v4(),
            Path::new(&mut upload_value.filename)
                .extension()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or("bin")
        );
        let filepath = format!("./uploads/{}", filename);

        // Save the file to the system
        if let Err(e) = File::create(filepath.clone()).and_then(|mut file| file.write_all(&image)) {
            log::error!("Failed to save file: {}", e);
            return Err(async_graphql::Error::new(format!("Failed to save file: {}", e)));
        }

        // Open the image to get its dimensions and format
        let img = match image::open(&filepath) {
            Ok(img) => img,
            Err(e) => {
                log::error!("Failed to open image: {}", e);
                return Err(async_graphql::Error::new(format!("Failed to open image: {}", e)));
            }
        };
        let (width, height) = img.dimensions();
        let image_format = image::guess_format(&image).unwrap(); //not supported

        // Convert the format to a MIME type string
        let media = match image_format {
            image::ImageFormat::Png => "image/png",
            image::ImageFormat::Jpeg => "image/jpeg",
            image::ImageFormat::Gif => "image/gif",
            image::ImageFormat::WebP => "image/webp",
            image::ImageFormat::Pnm => "image/pnm",
            image::ImageFormat::Tiff => "image/tiff",
            image::ImageFormat::Tga => "image/tga",
            image::ImageFormat::Dds => "image/dds",
            image::ImageFormat::Bmp => "image/bmp",
            image::ImageFormat::Ico => "image/ico",
            image::ImageFormat::Hdr => "image/hdr",
            image::ImageFormat::OpenExr => "image/openexr",
            image::ImageFormat::Farbfeld => "image/farbfeld",
            image::ImageFormat::Avif => "image/avif",
            image::ImageFormat::Qoi => "image/qoi",

            _ => "application/octet-stream", // Default to a generic binary format
        };

        let pool: &Pool<AsyncPgConnection> = ctx.data()?;
        let mut conn = pool.get().await?;

        // Insert the file's metadata into the database
        diesel::insert_into(images::table)
            .values((
                images::name.eq(upload_value.filename),
                images::file_path.eq(filepath),
                images::description.eq(None::<String>),
                images::exif_data.eq(None::<String>),
                images::format.eq(media),
                images::size.eq(image.len() as i32),
                images::width.eq(width as i32),
                images::height.eq(height as i32),
                images::created_at.eq(time_now),
                images::deleted_at.eq(None::<NaiveDateTime>),
            ))
            .execute(&mut conn)
            .await
            .map_err(|e| {
                log::error!("Failed to insert image into database:{}",e);
                PhotoError::DatabaseError
            })?;

        Ok(true)
    }
}


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
