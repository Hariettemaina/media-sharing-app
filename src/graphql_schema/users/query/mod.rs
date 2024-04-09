use async_graphql::MergedObject;

pub mod get_users;


use get_users::UserQuery;

#[derive(MergedObject, Default)]
pub struct UsersQuery(pub UserQuery);