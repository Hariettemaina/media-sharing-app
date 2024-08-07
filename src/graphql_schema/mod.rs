use async_graphql::SimpleObject;

pub mod images;
pub mod users;
pub mod videos;
pub mod payments;

use self::images::mutation::ImageMut;
use self::users::mutation::UserMut;
use self::videos::mutation::VideoMut;
use self::payments::mutation::MpesaMut;

#[derive(SimpleObject, Default)]
pub struct Mutation {
    pub users: UserMut,
    pub images: ImageMut,
    pub videos: VideoMut,
    pub payments: MpesaMut
}

use self::users::query::UsersQuery;

use self::images::query::MediaQuery;

#[derive(SimpleObject, Default)]
pub struct Query {
    pub users: UsersQuery,
    pub images: MediaQuery
}

use async_graphql::MergedSubscription;

#[derive(Default, MergedSubscription)]
pub struct Subscription(
    // pub users::subscription::new_user::GetNewUser,
    pub images::subscriptions::new_image::GetNewImage,
);

