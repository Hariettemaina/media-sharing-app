// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        middle_name -> Nullable<Varchar>,
        last_name -> Varchar,
        username -> Varchar,
        email_address -> Varchar,
        email_verification_code -> Uuid,
        email_verified -> Bool,
        email_verification_code_expiry -> Timestamp,
        password_hash -> Varchar,
        display_name -> Nullable<Varchar>,
        date_of_birth -> Date,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
