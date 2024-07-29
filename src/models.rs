use crate::schema::{email_address, images, transactions, users, videos};
use async_graphql::{InputObject, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use diesel::{query_builder::AsChangeset, Insertable, Queryable, Selectable};
use serde::Serialize;
use uuid::Uuid;

#[derive(InputObject, SimpleObject, Debug, Queryable, Selectable, Clone, AsChangeset)]
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

#[derive(SimpleObject, Queryable, Selectable, Debug, Serialize, Clone)]
#[diesel(table_name = images)]
pub struct Images {
    pub id: i32,
    pub name: String,
    pub file_path: String,
    pub description: Option<String>,
    pub exif_data: Option<String>,
    pub format: String,
    pub size: i32,
    pub width: i32,
    pub height: i32,
    pub created_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}
#[derive(Insertable)]
#[diesel(table_name = images)]
pub struct NewImage {
    pub name: String,
    pub file_path: String,
    pub description: Option<String>,
    pub exif_data: Option<String>,
    pub format: String,
    pub size: i32,
    pub width: i32,
    pub height: i32,
    pub created_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(SimpleObject, Queryable, Selectable, Debug, Serialize)]
#[diesel(table_name = videos)]
pub struct Videos {
    pub id: i32,
    pub title: String,
    pub codec_name: Option<String>,
    pub duration: Option<String>,
    pub file_path: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub bitrate: Option<String>,
    pub frame_rate: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = videos)]
pub struct NewVideos {
    pub title: String,
    pub codec_name: Option<String>,
    pub duration: Option<String>,
    pub file_path: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub bitrate: Option<String>,
    pub frame_rate: Option<String>,
    pub created_at: NaiveDateTime,
}


#[derive(InputObject, SimpleObject, Debug, Queryable, Selectable, Clone)]
#[diesel(table_name = transactions)]
pub struct Transaction {
    pub id: i32,
    pub user_id: i32,
    pub photo_id: i32,
    pub amount: i32,
    pub mpesa_number: Option<String>,
    pub mpesa_transaction_id: Option<String>,
    pub status: Option<String>,
    pub created_at: NaiveDateTime,    
}

// diesel::table! {
//     transactions (id) {
//         id -> Int4,
//         user_id -> Int4,
//         photo_id -> Int4,
//         amount -> Int4,
//         #[max_length = 20]
//         mpesa_number -> Nullable<Varchar>,
//         #[max_length = 255]
//         mpesa_transaction_id -> Nullable<Varchar>,
//         #[max_length = 50]
//         status -> Nullable<Varchar>,
//         created_at -> Timestamp,
//     }
// }