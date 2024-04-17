pub mod upload;

use async_graphql::MergedObject;
use upload::UploadMedia;



#[derive(MergedObject, Default)]
pub struct ImageMut(pub UploadMedia);
