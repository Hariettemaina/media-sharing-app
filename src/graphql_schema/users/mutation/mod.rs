pub mod login;
pub mod signup;

pub mod verify_email;

use async_graphql::MergedObject;
use login::Login;
use signup::AddUser;
use verify_email::Verify;

#[derive(MergedObject, Default)]
pub struct UserMut(pub AddUser, pub Login, pub Verify);
