pub mod login;
pub mod signup;
pub mod update;

pub mod verify_email;
pub mod logout;

use async_graphql::MergedObject;
use login::Login;
use signup::AddUser;
use verify_email::Verify;
use logout::Logout;

#[derive(MergedObject, Default)]
pub struct UserMut(pub AddUser, pub Login, pub Verify, pub Logout);
