use async_graphql::{Context, InputObject, Object, Result, Upload};
use chrono::{NaiveDateTime, Utc};
use diesel::ExpressionMethods;
use diesel_async::AsyncPgConnection;
use diesel_async::{pooled_connection::deadpool::Pool, RunQueryDsl};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use uuid::Uuid;

use crate::schema::images;

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
        let content = upload_value.content;
        content.read_to_end(&mut image)?;

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
        let mut file = File::create(filepath)?;
        file.write_all(&image)?;
        
        let pool: &Pool<AsyncPgConnection> = ctx.data()?;
        let mut conn = pool.get().await?;

        // Insert the file's metadata into the database
        diesel::insert_into(images::table)
            .values((
                images::name.eq(upload_value.filename),
                images::file_path.eq(filepath),
                images::description.eq(None::<String>),
                images::exif_data.eq(None::<String>),
                images::format.eq("image/jpeg"),
                images::size.eq(image.len() as i32),
                images::width.eq(0),
                images::height.eq(0),
                images::created_at.eq(time_now),
                images::deleted_at.eq(None::<NaiveDateTime>),
            ))
            .get_result(&mut conn)
            .await?;

        Ok(true)
    }
}


// the trait bound `(): Queryable<(Integer, diesel::sql_types::Text, diesel::sql_types::Text, 
   // diesel::sql_types::Nullable<diesel::sql_types::Text>, diesel::sql_types::Nullable<diesel::sql_types::Text>, 
    //diesel::sql_types::Text, Integer, Integer, Integer, diesel::sql_types::Timestamp, diesel::sql_types::Nullable<diesel::sql_types::Timestamp>),

    
// 2. *Media Upload*
//    - *Description:* Allow users to upload images and videos to the platform.
//    - *Expected Functionality:* Users can select and upload media files from their devices. Uploaded media is stored in the file system and metadata is stored in the database.
//    - *Criteria for Completion:* Users can successfully upload images and videos. Media files are stored in the file system and metadata is stored in the database.
//    - *Test Suites:*
//      - Test file upload functionality.
//      - Test database storage of metadata for uploaded media.
