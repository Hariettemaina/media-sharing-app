pub mod get;
use async_graphql::MergedObject;

use get::ImageQuery;


#[derive(MergedObject, Default)]
pub struct MediaQuery(pub ImageQuery);