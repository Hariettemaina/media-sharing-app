use crate::schema::{email_address, images, users};
use async_graphql::{InputObject, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use diesel::{Insertable, Queryable, Selectable};
use serde::Serialize;
use uuid::Uuid;

#[derive(InputObject, SimpleObject, Debug, Queryable, Selectable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub username: String,
    pub user_email: i32,
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
    pub user_email: i32,
    pub password_hash: String,
    pub display_name: Option<String>,
    pub date_of_birth: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(SimpleObject, Queryable, Selectable, Debug, Serialize)]
#[diesel(table_name = email_address)]
pub struct EmailAddress {
    pub id: i32,
    pub email: String,
    pub verification_code: Uuid,
    pub verification_code_expires_at: NaiveDateTime,
    pub verified_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = email_address)]
pub struct NewEmailAddress {
    pub email: String,
    pub verification_code: Uuid,
    pub verification_code_expires_at: NaiveDateTime,
}

#[derive(SimpleObject, Queryable, Selectable, Debug, Serialize)]
#[diesel(table_name = images)]
pub struct Images {
    pub id: i32,
    pub name: String,
    pub file_path: String,
    pub description:Option<String>,
    pub exif_data:Option<String>,
    pub format:String,
    pub size: i32,
    pub width: i32,
    pub height: i32,
    pub created_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>
}
#[derive(Insertable)]
#[diesel(table_name = images)]
pub struct  NewImage{
    pub name: String,
    pub file_path: String,
    pub description:Option<String>,
    pub exif_data:Option<String>,
    pub format:String,
    pub size: i32,
    pub width: i32,
    pub height: i32,
    pub created_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>
}

// create table if not exists images(
//     id serial primary key,
//     name varchar not null,
//     file_path varchar not null,
//     description varchar,
//     exif_data varchar,
//     format varchar not null,
//     size integer not null,
//     width integer not null,
//     height integer not null,
//     created_at timestamp not null default now(),
//     deleted_at timestamp
// );
