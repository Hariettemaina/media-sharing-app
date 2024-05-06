use async_graphql::{Context, InputObject, Object, Result, Upload};
use chrono::{NaiveDateTime, Utc};
use diesel::ExpressionMethods;
use diesel_async::AsyncPgConnection;
use diesel_async::{pooled_connection::deadpool::Pool, RunQueryDsl};
use image::{imageops::FilterType, GenericImageView};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use uuid::Uuid;

use crate::PhotoError;

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
        use crate::schema::images;

        let time_now = Utc::now().naive_utc();

        let mut image = Vec::new();
        let mut upload_value = input.image.value(ctx).unwrap();

        let mut content = upload_value.content; //file data
        if let Err(e) = content.read_to_end(&mut image) {
            log::error!("Failed to read image content: {}", e);
            return Err(async_graphql::Error::new(format!(
                "Failed to read image content: {}",
                e
            )));
        }

        // access the filename from the UploadValue    (each uploaded file has a unique name, even if two files have the same original name.uuid)
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

        // Check if the uploads directory exists, if not, create it
        if !Path::new(uploads_dir).exists() {
            fs::create_dir_all(uploads_dir).expect("Failed to create uploads directory");
        }

        // Check if the user-specific uploads directory exists, if not, create it
        if !Path::new(&user_uploads_dir).exists() {
            fs::create_dir_all(&user_uploads_dir)
                .expect("Failed to create user-specific uploads directory");
        }

        // Save the file to the system
        let filepath = format!("{}/{}", user_uploads_dir, filename);
        if let Err(e) = File::create(&filepath).and_then(|mut file| file.write_all(&image)) {
            log::error!("Failed to save file: {}", e);
            return Err(async_graphql::Error::new(format!(
                "Failed to save file: {}",
                e
            )));
        }

        // Open the image to get its dimensions and format
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
        let new_width = width / 2;
        let new_height = height / 2;
        let resized_img = img.resize(new_width, new_height, FilterType::Triangle);

        resized_img
            .save(&filepath)
            .expect("Failed to save resized imaged");
        let image_format = image::guess_format(&image).unwrap(); //not supported

        // Convert the format to a type string
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

            _ => "None",
        };

        let pool: &Pool<AsyncPgConnection> = ctx.data()?;
        let mut conn = pool.get().await?;

        // Insert the file's metadata into the database
        diesel::insert_into(images::table)
            .values((
                images::name.eq(upload_value.filename),
                images::file_path.eq(&filepath),
                images::description.eq(None::<String>),
                images::exif_data.eq(None::<String>),
                images::format.eq(media),
                images::size.eq(image.len() as i32),
                images::width.eq(resized_img.width() as i32),
                images::height.eq(resized_img.height() as i32),
                images::created_at.eq(time_now),
                images::deleted_at.eq(None::<NaiveDateTime>),
            ))
            .execute(&mut conn)
            .await
            .map_err(|e| {
                log::error!("Failed to insert image into database:{}", e);
                PhotoError::DatabaseError
            })?;

        Ok(filepath.clone())
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

// let (width, height) = img.dimensions();
// let new_width = width / 2;
// let new_height = height / 2;
// let resized_img = img.resize(new_width, new_height, FilterType::Triangle); // Improved filter for better quality

// // Save the resized image in JPEG format
// resized_img.save("resized_image.jpg").expect("Failed to save resized image");

// // use async_graphql::{Context, InputObject, Object, Result, Upload};
// use chrono::{NaiveDateTime, Utc};
// use diesel::ExpressionMethods;
// use diesel_async::AsyncPgConnection;
// use diesel_async::{pooled_connection::deadpool::Pool, RunQueryDsl};
// use image::{GenericImageView, GenericImage, ImageFormat, imageops::FilterType};
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
//     pub async fn upload(&self, ctx: &Context<'_>, input: UploadUserInput) -> Result<bool> {
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

//         // Generate a unique filename
//         let filename = format!(
//             "{}.{}",
//             Uuid::new_v4(),
//             Path::new(&mut upload_value.filename)
//                .extension()
//                .and_then(std::ffi::OsStr::to_str)
//                .unwrap_or("bin") //default value if path does not have an extension
//         );
//         let uploads_dir = "./uploads";
//         let user_uploads_dir = format!("{}/{}", uploads_dir, input.user_id);

//         // Ensure directories exist
//         if!Path::new(uploads_dir).exists() {
//             fs::create_dir_all(uploads_dir).expect("Failed to create uploads directory");
//         }
//         if!Path::new(&user_uploads_dir).exists() {
//             fs::create_dir_all(&user_uploads_dir)
//                .expect("Failed to create user-specific uploads directory");
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

//         // Resize the image
//         let (width, height) = img.dimensions();
//         let new_width = width / 2; // Example: halve the width
//         let new_height = height / 2; // Halve the height
//         let resized_img = img.resize(new_width, new_height, FilterType::Nearest);

//         // Convert the image to JPEG format for better compression
//         let jpeg_img = resized_img.into_raw();
//         let jpeg_img = image::encode_jpeg(&jpeg_img, 80); // 80 is the quality factor, adjust as needed

//         Ok(true)
//     }
// }
