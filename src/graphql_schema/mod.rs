use async_graphql::SimpleObject;

pub mod images;
pub mod users;
pub mod videos;

use self::images::mutation::ImageMut;
use self::users::mutation::UserMut;
use self::videos::mutation::VideoMut;

#[derive(SimpleObject, Default)]
pub struct Mutation {
    pub users: UserMut,
    pub images: ImageMut,
    pub videos: VideoMut
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
    pub users::subscription::new_user::GetNewUser,
);