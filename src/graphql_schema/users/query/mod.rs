use async_graphql::MergedObject;

pub mod get_users;
mod get_user_id;


use get_users::UserQuery;
use get_user_id::GetUser;

#[derive(MergedObject, Default)]
pub struct UsersQuery(pub UserQuery, pub GetUser);