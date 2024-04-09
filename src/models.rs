use async_graphql::{InputObject, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


use crate::schema::users;


#[derive(SimpleObject, Queryable, Selectable, Debug, Serialize, Deserialize,)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub username: String,
    pub email_address: i32,
    pub password_hash: String,
    pub display_name: Option<String>,
    pub date_of_birth: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}