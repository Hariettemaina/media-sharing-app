pub mod new_image;

use async_graphql::MergedSubscription;
use new_image::GetNewImage;


#[derive(MergedSubscription, Default)]
pub struct Subscription(GetNewImage);
