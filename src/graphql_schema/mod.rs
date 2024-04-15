use async_graphql::SimpleObject;

pub mod users;
pub mod images;

use crate::graphql_schema::users::mutation::UserMut;

#[derive(SimpleObject, Default)]
pub struct Mutation {
    pub users: UserMut,
}

use crate::graphql_schema::users::query::UsersQuery;

#[derive(SimpleObject, Default)]
pub struct Query {
    pub users: UsersQuery,
}
