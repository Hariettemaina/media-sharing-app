use crate::schema::users;
use async_graphql::{InputObject, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use diesel::{Insertable, Queryable, Selectable};
use uuid::Uuid;

#[derive(InputObject, SimpleObject, Debug, Queryable, Selectable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub username: String,
    pub email_address: String,
    pub email_verification_code: Uuid,
    pub email_verified: bool,
    pub email_verification_code_expiry: NaiveDateTime,
    pub password_hash: String,
    pub display_name: Option<String>,
    pub date_of_birth: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub username: String,
    pub email_address: String,
    pub email_verification_code: Uuid,
    pub email_verified: bool,
    pub email_verification_code_expiry: NaiveDateTime,
    pub password_hash: String,
    pub display_name: Option<String>,
    pub date_of_birth: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
// id -> Int4,
//         first_name -> Varchar,
//         middle_name -> Nullable<Varchar>,
//         last_name -> Varchar,
//         username -> Varchar,
//         email_address -> Varchar,
//         email_verification_code -> Uuid,
//         email_verified -> Bool,
//         email_verification_code_expiry -> Timestamp,
//         password_hash -> Varchar,
//         display_name -> Nullable<Varchar>,
//         date_of_birth -> Date,
//         created_at -> Timestamp,
//         updated_at -> Timestamp,