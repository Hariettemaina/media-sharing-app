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
    images (id) {
        id -> Int4,
        name -> Varchar,
        file_path -> Varchar,
        description -> Nullable<Varchar>,
        exif_data -> Nullable<Varchar>,
        format -> Varchar,
        size -> Int4,
        width -> Int4,
        height -> Int4,
        created_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    transactions (id) {
        id -> Int4,
        user_id -> Int4,
        photo_id -> Int4,
        amount -> Int8,
        #[max_length = 20]
        mpesa_number -> Nullable<Varchar>,
        #[max_length = 255]
        mpesa_transaction_id -> Nullable<Varchar>,
        #[max_length = 50]
        status -> Nullable<Varchar>,
        created_at -> Timestamp,
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

diesel::table! {
    videos (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Varchar>,
        codec_name -> Nullable<Varchar>,
        duration -> Nullable<Varchar>,
        file_path -> Varchar,
        width -> Nullable<Int4>,
        height -> Nullable<Int4>,
        bitrate -> Nullable<Varchar>,
        frame_rate -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

diesel::joinable!(transactions -> images (photo_id));
diesel::joinable!(transactions -> users (user_id));
diesel::joinable!(users -> email_address (user_email));

diesel::allow_tables_to_appear_in_same_query!(
    email_address,
    images,
    transactions,
    users,
    videos,
);
