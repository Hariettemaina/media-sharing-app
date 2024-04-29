use async_graphql::{Context, InputObject, Object, Result, Upload};
use chrono::Utc;
use diesel::ExpressionMethods;
use diesel_async::AsyncPgConnection;
use diesel_async::{pooled_connection::deadpool::Pool, RunQueryDsl};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use uuid::Uuid;
use crate::PhotoError;

#[derive(Default)]
pub struct UploadVideo;

#[derive(InputObject)]
pub struct VideoUserInput {
    pub video: Upload,
    pub user_id: i32,
}

#[Object]
impl UploadVideo {
    pub async fn upload_video(&self, ctx: &Context<'_>, input: VideoUserInput) -> Result<bool> {
        use crate::schema::videos;

        let time_now = Utc::now().naive_utc();
        let mut video = Vec::new();
        let mut upload_value = input.video.value(ctx).unwrap();

        let mut content = upload_value.content;
        if let Err(e) = content.read_to_end(&mut video) {
            log::error!("Failed to read video content: {}", e);
            return Err(async_graphql::Error::new(format!(
                "Failed to read video content:{}",
                e
            )));
        }

        let filename = format!(
            "{}.{}",
            Uuid::new_v4(),
            Path::new(&mut upload_value.filename)
                .extension()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or("bin")
        );

        let uploads_dir = "./uploads";
        let user_uploads_dir = format!("{}/{}", uploads_dir, input.user_id);

        if !Path::new(uploads_dir).exists() {
            fs::create_dir_all(&uploads_dir).expect("Failed to create uploads directory");
        }
        let filepath = format!("{}/{}", user_uploads_dir, filename);
        if let Err(e) = File::create(&filepath).and_then(|mut file| file.write_all(&video)) {
            log::error!("Failed to save file: {}", e);
            return Err(async_graphql::Error::new(format!(
                "Failed to save file: {}",
                e
            )));
        }

        //let (height, width) = todo!();

        let pool: &Pool<AsyncPgConnection> = ctx.data()?;
        let mut conn = pool.get().await?;

        diesel::insert_into(videos::table)
            .values((
                videos::title.eq(upload_value.filename),
                videos::description.eq(None::<String>),
                videos::codec_name.eq(None::<String>),
                videos::duration.eq(None::<String>),
                videos::file_path.eq(filepath),
                videos::width.eq(None::<i32>),
                videos::height.eq(None::<i32>),
                videos::bitrate.eq(None::<String>),
                videos::frame_rate.eq(None::<String>),
                videos::created_at.eq(time_now),
            ))
            .execute(&mut conn)
            .await
            .map_err(|e| {
                log::error!("Failed to insert image into database:{}", e);
                PhotoError::DatabaseError
            })?;

        Ok(true)
    }
}


