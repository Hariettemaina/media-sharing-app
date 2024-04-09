pub mod login;
pub mod signup;

pub mod verify_email;

use async_graphql::MergedObject;
use login::Login;
use signup::AddUser;

#[derive(MergedObject, Default)]
pub struct UserMut(pub AddUser, pub Login);
