#[cfg(test)]
mod tests {
    use crate::graphql_schema::images::mutation::upload::{UploadMedia, UploadUserInput};

    use async_graphql::{Context, EmptySubscription, Schema, Upload, UploadValue};
    use diesel_async::pooled_connection::deadpool::Pool;
    use diesel_async::pooled_connection::AsyncDieselConnectionManager;
    use std::env;
    use std::fs::File;
    use std::io::Write;

    use tokio::fs;

    struct MediaUpload {
        filename: String,
        content_type: Option<String>,
        content: Vec<u8>,
    }

    impl MediaUpload {
        fn new(filename: String, content_type: Option<String>, content: Vec<u8>) -> Self {
            Self {
                filename,
                content_type,
                content,
            }
        }
    }


    
    impl MediaUpload {
        async fn value(
            &self,
            _ctx: &Context<'_>,
        ) -> Result<UploadValue, async_graphql::Error> {
            let mut temp_file =
                tempfile::NamedTempFile::new().expect("Failed to create temporary file");
            temp_file
                .write_all(&self.content)
                .expect("Failed to write to temporary file");

                let file = File::open(temp_file.path()).expect("Failed to open temporary file");

            Ok(UploadValue {
                filename: self.filename.clone(),
                content_type: self.content_type.clone(),
                content: file
            })
        }
    }
    
    #[tokio::test]
    async fn test_upload_media() {
        dotenvy::dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let config =
            AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
        let pool = Pool::builder(config).build().unwrap();

        let schema = Schema::build(UploadMedia, UploadMedia, EmptySubscription)
            .data(pool)
            .finish();

        // Create a temporary file to simulate an uploaded file
        let temp_file_path = "./uploads/test_upload.jpg";
        let mut file = File::create(temp_file_path).expect("Failed to create temporary file");
        file.write_all(b"test image content")
            .expect("Failed to write to temporary file");

        let mock_upload = Upload::new(
            "test_upload.jpg".to_string(),
            Some("image/jpeg".to_string()),
            b"test image content".to_vec(),
        );

        let input = UploadUserInput {
            image: mock_upload,
            user_id: 1,
        };

        let result = schema
            .execute(
                r#"
            mutation UploadMedia($input: UploadUserInput!) {
                upload(input: $input)
            }
        "#,
            )
            .await;

        assert!(result.is_ok());

        // Cleanup: Remove the temporary file
        fs::remove_file(temp_file_path)
            .await
            .expect("Failed to remove temporary file");
    }
}
// async fn as_upload(&self, ctx: &Context<'_>) -> Result<Upload, async_graphql::Error> {
//     let upload_value = self.as_upload_value(ctx).await?;
//     Ok(Upload(upload_value))
// }