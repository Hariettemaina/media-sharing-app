use async_graphql::{Context, InputObject, Object, Result, Upload};
use chrono::Utc;
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
    async fn process_image(
        &self,
        ctx: &Context<'_>,
        input: ImageUploadInput,
    ) -> Result<String, PhotoError> {
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

            let image_path = format!(
                "image_uploads/{}-{}.{}",
                upload_value.filename, time_now, extension
            );

            let mut file = fs::File::create(&image_path).map_err(|err| {
                log::error!("Failed to create image file: {}", err);
                PhotoError::Internal(InternalError::Io(err))
            })?;

            file.write_all(&image_data).map_err(|err| {
                log::error!("Failed to write image data to file: {}", err);
                PhotoError::Internal(InternalError::Io(err))
            })?;

            let img = image::open(&image_path).map_err(|err| {
                log::error!("Failed to open image file: {}", err);
                let io_error =
                    std::io::Error::new(std::io::ErrorKind::Other, "Failed to open image");
                let internal_error = InternalError::Io(io_error);
                PhotoError::Internal(internal_error)
            })?;

            let (width, height) = img.dimensions();
            let image_format = ImageFormat::from_extension(extension).unwrap();

            log::info!(
                "Image width: {}, height: {}, format: {:#?}",
                width as i32,
                height as i32,
                image_format
            );

            //  different viewports
            let target_viewport_sizes = vec![480, 768, 1024, 1024, 1440];
            for target_size in target_viewport_sizes.iter() {
                let resized_img = img.resize(
                    *target_size,
                    *target_size,
                    image::imageops::FilterType::Nearest,
                );
                let output_path = format!("resized_{}_{}.{}", image_path, target_size, extension);
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

            Ok(image_path)
        } else {
            Err(PhotoError::UserAccountAlreadyExists)
        }
    }
}

// mobile devices481px
// ipads,tablets  481px-768px
// desktop 1024px+
// laptops769 - 1024px
// exta large screens 1440px+
