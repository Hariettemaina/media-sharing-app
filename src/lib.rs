pub mod models;
pub mod schema;
pub mod graphql_schema;
mod error;
use actix_session::Session;
pub use error::{InternalError, PhotoError};
use send_wrapper::SendWrapper;
pub mod password;
pub mod mailer;
pub mod tests;
pub mod services;
// pub mod rabbitmq;
use crate::graphql_schema::users::mutation::login::Shared;


pub struct RequestContext {
    pub session: Shared<SendWrapper<Session>>,
}


