use async_graphql::{Context, InputObject, Object, Result};
use chrono::{Duration,NaiveDateTime,Utc};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncConnection,RunQueryDsl};

#[derive(Default)]
pub struct UploadMedia;

#[Object]
impl UploadMedia{
    pub async fn upload(&self, ctx: &Context<'_>)-> Result<bool>{

    }
}

// 2. *Media Upload*
//    - *Description:* Allow users to upload images and videos to the platform.
//    - *Expected Functionality:* Users can select and upload media files from their devices. Uploaded media is stored in the file system and metadata is stored in the database.
//    - *Criteria for Completion:* Users can successfully upload images and videos. Media files are stored in the file system and metadata is stored in the database.
//    - *Test Suites:*
//      - Test file upload functionality.
//      - Test database storage of metadata for uploaded media.
