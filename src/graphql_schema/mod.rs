use async_graphql::SimpleObject;

pub mod images;
pub mod users;
pub mod videos;

use crate::graphql_schema::images::mutation::ImageMut;
use crate::graphql_schema::users::mutation::UserMut;
use crate::graphql_schema::videos::mutation::VideoMut;

#[derive(SimpleObject, Default)]
pub struct Mutation {
    pub users: UserMut,
    pub images: ImageMut,
    pub videos: VideoMut
}

use crate::graphql_schema::users::query::UsersQuery;

#[derive(SimpleObject, Default)]
pub struct Query {
    pub users: UsersQuery,
}
