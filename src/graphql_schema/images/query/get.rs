use crate::{schema::images, PhotoError};
use async_graphql::{Context, Object, Result};
use diesel::query_dsl::methods::SelectDsl;
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};

#[derive(Default)]
pub struct ImageQuery;



#[Object]
impl ImageQuery {
    pub async fn get_images<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<String>> {
        let pool = ctx.data::<Pool<AsyncPgConnection>>()?;
        let mut connection = pool.get().await?;

        let images = images::table
            .select(images::file_path)
            .load::<String>(&mut connection)
            .await
            .map_err(|e| {
                log::error!("Failed to fetch images: {}", e);
                PhotoError::DatabaseError
            })?;

        Ok(images)
    }
}
