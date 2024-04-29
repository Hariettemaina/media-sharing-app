pub mod videos;


use async_graphql::MergedObject;
use videos::UploadVideo;



#[derive(MergedObject, Default)]
pub struct VideoMut(pub UploadVideo);
