// @generated automatically by Diesel CLI.

diesel::table! {
    email_address (id) {
        id -> Int4,
        email -> Varchar,
        verification_code -> Uuid,
        verification_code_expires_at -> Timestamp,
        verified_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        middle_name -> Nullable<Varchar>,
        last_name -> Varchar,
        username -> Varchar,
        user_email -> Int4,
        password_hash -> Varchar,
        display_name -> Nullable<Varchar>,
        date_of_birth -> Date,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(users -> email_address (user_email));

diesel::allow_tables_to_appear_in_same_query!(
    email_address,
    users,
);
