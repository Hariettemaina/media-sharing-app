use async_graphql::MergedSubscription;


use new_user::GetNewUser;
pub mod new_user;


#[derive(MergedSubscription, Default)]
pub struct Subscription(GetNewUser);
