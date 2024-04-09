// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        middle_name -> Nullable<Varchar>,
        last_name -> Varchar,
        username -> Varchar,
        email_address -> Varchar,
        password_hash -> Varchar,
        display_name -> Nullable<Varchar>,
        date_of_birth -> Date,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
