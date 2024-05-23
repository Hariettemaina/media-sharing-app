use crate::schema::images;
use async_graphql::futures_util::TryFutureExt;
use async_graphql::{Context, InputObject, Object, Result, Upload};
use chrono::{NaiveDateTime, Utc};
use diesel::ExpressionMethods;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use image::{GenericImageView, ImageFormat};
use std::ffi::OsStr;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;

use crate::{InternalError, PhotoError};

#[derive(Default)]
pub struct ImageProcessor;

#[derive(InputObject)]
pub struct ImageUploadInput {
    image: Upload,
}

#[Object]
impl ImageProcessor {
    async fn process_image(&self, ctx: &Context<'_>, input: ImageUploadInput) -> Result<String> {
        dotenvy::dotenv().ok();

        let time_now = Utc::now().naive_local();

        let mut image_data = Vec::new();

        let mut upload_value = input.image.value(ctx).map_err(|err| {
            log::error!("Failed to get image value: {}", err);
            PhotoError::Internal(InternalError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to get image value",
            )))
        })?;

        upload_value
            .content
            .read_to_end(&mut image_data)
            .map_err(|err| {
                log::error!("Failed to read image content: {}", err);
                PhotoError::Internal(InternalError::Io(err))
            })?;

        if let Some(extension) = Path::new(&upload_value.filename)
            .extension()
            .and_then(OsStr::to_str)
        {
            log::info!("File extension: {:#?}", extension);
            log::info!("Image data length: {:#?}", image_data.len());

            let _ = std::env::var("IMAGE_UPLOADS_DIR")
                .map(fs::create_dir_all)
                .map_err(|_| {
                    log::error!("Failed to create directory for image uploads");
                    PhotoError::DatabaseError
                })?;

            let filepath = format!(
                "uploads/{}-{}.{}",
                upload_value.filename, time_now, extension
            );

            let mut file = fs::File::create(&filepath).map_err(|err| {
                log::error!("Failed to create image file: {}", err);
                PhotoError::Internal(InternalError::Io(err))
            })?;

            file.write_all(&image_data).map_err(|err| {
                log::error!("Failed to write image data to file: {}", err);
                PhotoError::Internal(InternalError::Io(err))
            })?;

            let img = image::open(&filepath).map_err(|err| {
                log::error!("Failed to open image file: {}", err);
                let io_error =
                    std::io::Error::new(std::io::ErrorKind::Other, "Failed to open image");
                let internal_error = InternalError::Io(io_error);
                PhotoError::Internal(internal_error)
            })?;

            let (width, height) = img.dimensions();
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

            log::info!(
                "Image width: {}, height: {}, format: {:#?}",
                width as i32,
                height as i32,
                image_format
            );

            //  different viewports
            let target_viewport_sizes = vec![480, 768, 1024, 1024, 1440, 2650];
            for target_size in target_viewport_sizes.iter() {
                let resized_img = img.resize(
                    *target_size,
                    *target_size,
                    image::imageops::FilterType::Nearest,
                );
                let output_path = format!("resized_{}_{}.{}", filepath, target_size, extension);
                let mut output_file = fs::File::create(&output_path).map_err(|err| {
                    log::error!("Failed to create resized image file: {}", err);
                    PhotoError::Internal(InternalError::Io(err))
                })?;
                resized_img
                    .write_to(&mut output_file, image::ImageFormat::Png)
                    .map_err(|err| {
                        log::error!("Failed to write resized image data to file: {}", err);
                        PhotoError::Internal(err.into())
                    })?;
            }

            let pool: &Pool<AsyncPgConnection> = ctx.data()?;
            let mut conn = pool.get().await?;

            diesel::insert_into(images::table)
                .values((
                    images::name.eq(upload_value.filename.clone()),
                    images::file_path.eq(&filepath),
                    images::description.eq(Some("Processed image".to_string())),
                    images::exif_data.eq(None::<String>),
                    images::format.eq(image_format_str),
                    images::size.eq(image_data.len() as i32),
                    images::width.eq(width as i32),
                    images::height.eq(height as i32),
                    images::created_at.eq(time_now),
                    images::deleted_at.eq(None::<NaiveDateTime>),
                ))
                .execute(&mut conn)
                .map_err(|e| {
                    log::error!("Failed to insert image metadata: {}", e);
                    PhotoError::DatabaseError
                })
                .await?;

            Ok(filepath)
        } else {
            log::error!("Invalid extension: {}", upload_value.filename);
            Err(PhotoError::InvalidExtension.into())
        }
    }
}

// mobile devices481px
// ipads,tablets  481px-768px
// desktop 1024px+
// laptops769 - 1024px
// exta large screens 1440px+
